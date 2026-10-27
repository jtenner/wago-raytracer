#![no_std]
mod imports;
mod tracer;
mod wire;

use core::{ffi::c_void, mem::{size_of, zeroed}, ptr::{addr_of, addr_of_mut, null}};
use imports::*;
use tracer::{WIDTH,HEIGHT,COUNT};
use wire::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    #[cfg(target_arch="wasm32")]
    core::arch::wasm32::unreachable();
    #[cfg(not(target_arch="wasm32"))]
    loop { core::hint::spin_loop(); }
}

fn p<T>(x: &T) -> *const c_void { x as *const T as *const c_void }
fn m<T>(x: &mut T) -> *mut c_void { x as *mut T as *mut c_void }
fn offset<T>(x: &T) -> u32 { x as *const T as usize as u32 }
// Generated wire types contain only integers, floats, and fixed arrays.
fn zero<T: Copy>() -> T { unsafe { zeroed() } }
macro_rules! check { ($s:expr,$call:expr) => {{ let r=$call; if r!=0 { $s.error_line=line!(); return Err(r); } }}; }
macro_rules! require { ($s:expr,$test:expr) => { if !$test { $s.error_line=line!(); return Err(-100000); } }; }

const LINEAR_BYTES: u32=(COUNT*16) as u32;
const FRAME_BYTES: u32=(COUNT*4) as u32;
static mut READBACK: [u8; COUNT*4]=[0;COUNT*4];
#[repr(C,align(4))]
struct Shader<const N: usize>([u8;N]);
static VERT: Shader<{include_bytes!("../build/fullscreen.spv").len()}> = Shader(*include_bytes!("../build/fullscreen.spv"));
static FRAG: Shader<{include_bytes!("../build/present.spv").len()}> = Shader(*include_bytes!("../build/present.spv"));

struct State {
    instance:u64,device:u64,queue:u64,upload:u64,upload_mem:u64,output:u64,output_mem:u64,
    image:u64,image_mem:u64,view:u64,pass:u64,framebuffer:u64,vert:u64,frag:u64,
    set_layout:u64,pipeline_layout:u64,pipeline:u64,descriptor_pool:u64,pool:u64,error_line:u32,
    properties:VkPhysicalDeviceProperties,memory:VkPhysicalDeviceMemoryProperties,
}
static mut STATE: State=unsafe {zeroed()};

unsafe fn allocate(s:&mut State,req:VkMemoryRequirements,flags:u32) -> Result<u64,i32> {
    for i in 0..s.memory.memoryTypeCount {
        if req.memoryTypeBits&(1<<i)!=0 && s.memory.memoryTypes[i as usize].propertyFlags&flags==flags {
            let a=VkMemoryAllocateInfo{sType:VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,allocationSize:req.size,memoryTypeIndex:i,..zero()};
            let mut memory=0; check!(s,vkAllocateMemory(s.device,p(&a),null(),&mut memory)); return Ok(memory);
        }
    }
    s.error_line=line!(); Err(-100000)
}
unsafe fn buffer(s:&mut State,bytes:u64,usage:u32,upload:bool) -> Result<(),i32> {
    let b=VkBufferCreateInfo{sType:VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,size:bytes,usage,..zero()};
    let mut handle=0; check!(s,vkCreateBuffer(s.device,p(&b),null(),&mut handle));
    if upload {s.upload=handle;}else{s.output=handle;}
    let mut req:VkMemoryRequirements=zero(); vkGetBufferMemoryRequirements(s.device,handle,m(&mut req));
    let memory=allocate(s,req,VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT|VK_MEMORY_PROPERTY_HOST_COHERENT_BIT)?;
    if upload {s.upload_mem=memory;}else{s.output_mem=memory;}
    check!(s,vkBindBufferMemory(s.device,handle,memory,0)); Ok(())
}

unsafe fn render_inner(s:&mut State,require_gpu:bool) -> Result<(),i32> {
    require!(s,s.instance==0); require!(s,abiVersion()==1);
    let app=VkApplicationInfo{sType:VK_STRUCTURE_TYPE_APPLICATION_INFO,pApplicationName:offset(b"Wago ray tracer\0"),apiVersion:VK_API_VERSION_1_0,..zero()};
    let ic=VkInstanceCreateInfo{sType:VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,pApplicationInfo:offset(&app),..zero()};
    check!(s,vkCreateInstance(p(&ic),null(),&mut s.instance));
    let mut devices=[0u64;16];let mut count=16;
    check!(s,vkEnumeratePhysicalDevices(s.instance,&mut count,devices.as_mut_ptr()));require!(s,count>0);
    let mut physical=0;let mut family=0;let mut best=-1;
    for &dev in &devices[..count as usize] {
        let mut prop:VkPhysicalDeviceProperties=zero(); vkGetPhysicalDeviceProperties(dev,m(&mut prop));
        if require_gpu && prop.deviceType!=VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU && prop.deviceType!=VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU {continue;}
        let mut n=0; vkGetPhysicalDeviceQueueFamilyProperties(dev,&mut n,core::ptr::null_mut());require!(s,n<=64);
        let mut families:[VkQueueFamilyProperties;64]=[zero();64];vkGetPhysicalDeviceQueueFamilyProperties(dev,&mut n,families.as_mut_ptr().cast());
        for j in 0..n as usize { if families[j].queueCount>0 && families[j].queueFlags&VK_QUEUE_GRAPHICS_BIT!=0 {
            let score=if prop.deviceType==VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU {3}else if prop.deviceType==VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU {2}else{1};
            if score>best {best=score;physical=dev;s.properties=prop;family=j as u32;}break;
        }}
    }
    require!(s,physical!=0);vkGetPhysicalDeviceMemoryProperties(physical,m(&mut s.memory));
    let priority=1f32;
    let qi=VkDeviceQueueCreateInfo{sType:VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,queueFamilyIndex:family,queueCount:1,pQueuePriorities:offset(&priority),..zero()};
    let dc=VkDeviceCreateInfo{sType:VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,queueCreateInfoCount:1,pQueueCreateInfos:offset(&qi),..zero()};
    check!(s,vkCreateDevice(physical,p(&dc),null(),&mut s.device));vkGetDeviceQueue(s.device,family,0,&mut s.queue);

    tracer::postcard();
    buffer(s,LINEAR_BYTES as u64,VK_BUFFER_USAGE_STORAGE_BUFFER_BIT,true)?;
    buffer(s,FRAME_BYTES as u64,VK_BUFFER_USAGE_TRANSFER_DST_BIT,false)?;
    let mut mapped=0;check!(s,vkMapMemory(s.device,s.upload_mem,0,LINEAR_BYTES as u64,0,&mut mapped));
    writeMapped(mapped,addr_of!(tracer::PIXELS).cast(),LINEAR_BYTES);vkUnmapMemory(s.device,s.upload_mem);
    let im=VkImageCreateInfo{sType:VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,imageType:VK_IMAGE_TYPE_2D,format:VK_FORMAT_R8G8B8A8_UNORM,
        extent:VkExtent3D{width:WIDTH as u32,height:HEIGHT as u32,depth:1},mipLevels:1,arrayLayers:1,samples:VK_SAMPLE_COUNT_1_BIT,
        tiling:VK_IMAGE_TILING_OPTIMAL,usage:VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT|VK_IMAGE_USAGE_TRANSFER_SRC_BIT,..zero()};
    check!(s,vkCreateImage(s.device,p(&im),null(),&mut s.image));
    let mut req:VkMemoryRequirements=zero();vkGetImageMemoryRequirements(s.device,s.image,m(&mut req));
    s.image_mem=allocate(s,req,VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT)?;check!(s,vkBindImageMemory(s.device,s.image,s.image_mem,0));
    let iv=VkImageViewCreateInfo{sType:VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,image:s.image,viewType:VK_IMAGE_VIEW_TYPE_2D,format:im.format,
        subresourceRange:VkImageSubresourceRange{aspectMask:VK_IMAGE_ASPECT_COLOR_BIT,levelCount:1,layerCount:1,..zero()},..zero()};
    check!(s,vkCreateImageView(s.device,p(&iv),null(),&mut s.view));
    let attachment=VkAttachmentDescription{format:im.format,samples:VK_SAMPLE_COUNT_1_BIT,loadOp:VK_ATTACHMENT_LOAD_OP_DONT_CARE,
        storeOp:VK_ATTACHMENT_STORE_OP_STORE,stencilLoadOp:VK_ATTACHMENT_LOAD_OP_DONT_CARE,stencilStoreOp:VK_ATTACHMENT_STORE_OP_DONT_CARE,
        initialLayout:VK_IMAGE_LAYOUT_UNDEFINED,finalLayout:VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,..zero()};
    let reference=VkAttachmentReference{attachment:0,layout:VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL};
    let subpass=VkSubpassDescription{pipelineBindPoint:VK_PIPELINE_BIND_POINT_GRAPHICS,colorAttachmentCount:1,pColorAttachments:offset(&reference),..zero()};
    let dependency=VkSubpassDependency{srcSubpass:0,dstSubpass:VK_SUBPASS_EXTERNAL,srcStageMask:VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
        dstStageMask:VK_PIPELINE_STAGE_TRANSFER_BIT,srcAccessMask:VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,dstAccessMask:VK_ACCESS_TRANSFER_READ_BIT,..zero()};
    let rp=VkRenderPassCreateInfo{sType:VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,attachmentCount:1,pAttachments:offset(&attachment),
        subpassCount:1,pSubpasses:offset(&subpass),dependencyCount:1,pDependencies:offset(&dependency),..zero()};
    check!(s,vkCreateRenderPass(s.device,p(&rp),null(),&mut s.pass));
    let fb=VkFramebufferCreateInfo{sType:VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,renderPass:s.pass,attachmentCount:1,pAttachments:offset(&s.view),
        width:WIDTH as u32,height:HEIGHT as u32,layers:1,..zero()};
    check!(s,vkCreateFramebuffer(s.device,p(&fb),null(),&mut s.framebuffer));
    let mut sm=VkShaderModuleCreateInfo{sType:VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,codeSize:size_of_val_shader(&VERT),pCode:offset(&VERT),..zero()};
    check!(s,vkCreateShaderModule(s.device,p(&sm),null(),&mut s.vert));sm.codeSize=size_of_val_shader(&FRAG);sm.pCode=offset(&FRAG);
    check!(s,vkCreateShaderModule(s.device,p(&sm),null(),&mut s.frag));
    let binding=VkDescriptorSetLayoutBinding{binding:0,descriptorType:VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,descriptorCount:1,stageFlags:VK_SHADER_STAGE_FRAGMENT_BIT,..zero()};
    let dl=VkDescriptorSetLayoutCreateInfo{sType:VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,bindingCount:1,pBindings:offset(&binding),..zero()};
    check!(s,vkCreateDescriptorSetLayout(s.device,p(&dl),null(),&mut s.set_layout));
    let pl=VkPipelineLayoutCreateInfo{sType:VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,setLayoutCount:1,pSetLayouts:offset(&s.set_layout),..zero()};
    check!(s,vkCreatePipelineLayout(s.device,p(&pl),null(),&mut s.pipeline_layout));
    let stages=[VkPipelineShaderStageCreateInfo{sType:VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,stage:VK_SHADER_STAGE_VERTEX_BIT,module:s.vert,pName:offset(b"main\0"),..zero()},
        VkPipelineShaderStageCreateInfo{sType:VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,stage:VK_SHADER_STAGE_FRAGMENT_BIT,module:s.frag,pName:offset(b"main\0"),..zero()}];
    let vi=VkPipelineVertexInputStateCreateInfo{sType:VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,..zero()};
    let ia=VkPipelineInputAssemblyStateCreateInfo{sType:VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,topology:VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,..zero()};
    let viewport=VkViewport{x:0.,y:0.,width:WIDTH as f32,height:HEIGHT as f32,minDepth:0.,maxDepth:1.};
    let scissor=VkRect2D{offset:VkOffset2D{x:0,y:0},extent:VkExtent2D{width:WIDTH as u32,height:HEIGHT as u32}};
    let vp=VkPipelineViewportStateCreateInfo{sType:VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,viewportCount:1,pViewports:offset(&viewport),scissorCount:1,pScissors:offset(&scissor),..zero()};
    let rs=VkPipelineRasterizationStateCreateInfo{sType:VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,lineWidth:1.,..zero()};
    let ms=VkPipelineMultisampleStateCreateInfo{sType:VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,rasterizationSamples:VK_SAMPLE_COUNT_1_BIT,..zero()};
    let blend=VkPipelineColorBlendAttachmentState{colorWriteMask:15,..zero()};
    let cb=VkPipelineColorBlendStateCreateInfo{sType:VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,attachmentCount:1,pAttachments:offset(&blend),..zero()};
    let gp=VkGraphicsPipelineCreateInfo{sType:VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,stageCount:2,pStages:offset(&stages),pVertexInputState:offset(&vi),
        pInputAssemblyState:offset(&ia),pViewportState:offset(&vp),pRasterizationState:offset(&rs),pMultisampleState:offset(&ms),pColorBlendState:offset(&cb),
        layout:s.pipeline_layout,renderPass:s.pass,basePipelineIndex:u32::MAX,..zero()};
    check!(s,vkCreateGraphicsPipelines(s.device,0,1,p(&gp),null(),&mut s.pipeline));
    let size=VkDescriptorPoolSize{r#type:VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,descriptorCount:1};
    let dp=VkDescriptorPoolCreateInfo{sType:VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,maxSets:1,poolSizeCount:1,pPoolSizes:offset(&size),..zero()};
    check!(s,vkCreateDescriptorPool(s.device,p(&dp),null(),&mut s.descriptor_pool));
    let ds=VkDescriptorSetAllocateInfo{sType:VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,descriptorPool:s.descriptor_pool,descriptorSetCount:1,pSetLayouts:offset(&s.set_layout),..zero()};
    let mut set=0;check!(s,vkAllocateDescriptorSets(s.device,p(&ds),&mut set));
    let db=VkDescriptorBufferInfo{buffer:s.upload,offset:0,range:LINEAR_BYTES as u64};
    let write=VkWriteDescriptorSet{sType:VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,dstSet:set,descriptorCount:1,descriptorType:VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,pBufferInfo:offset(&db),..zero()};
    vkUpdateDescriptorSets(s.device,1,p(&write),0,null());
    let cp=VkCommandPoolCreateInfo{sType:VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,queueFamilyIndex:family,..zero()};
    check!(s,vkCreateCommandPool(s.device,p(&cp),null(),&mut s.pool));
    let ca=VkCommandBufferAllocateInfo{sType:VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,commandPool:s.pool,commandBufferCount:1,..zero()};
    let mut command=0;check!(s,vkAllocateCommandBuffers(s.device,p(&ca),&mut command));
    let begin=VkCommandBufferBeginInfo{sType:VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,flags:VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,..zero()};
    check!(s,vkBeginCommandBuffer(command,p(&begin)));
    let rb=VkRenderPassBeginInfo{sType:VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,renderPass:s.pass,framebuffer:s.framebuffer,renderArea:scissor,..zero()};
    vkCmdBeginRenderPass(command,p(&rb),VK_SUBPASS_CONTENTS_INLINE);vkCmdBindPipeline(command,VK_PIPELINE_BIND_POINT_GRAPHICS,s.pipeline);
    vkCmdBindDescriptorSets(command,VK_PIPELINE_BIND_POINT_GRAPHICS,s.pipeline_layout,0,1,p(&set),0,null());
    vkCmdDraw(command,3,1,0,0);vkCmdEndRenderPass(command);
    let copy=VkBufferImageCopy{imageSubresource:VkImageSubresourceLayers{aspectMask:VK_IMAGE_ASPECT_COLOR_BIT,layerCount:1,..zero()},imageExtent:im.extent,..zero()};
    vkCmdCopyImageToBuffer(command,s.image,VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,s.output,1,p(&copy));
    let barrier=VkBufferMemoryBarrier{sType:VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,srcAccessMask:VK_ACCESS_TRANSFER_WRITE_BIT,dstAccessMask:VK_ACCESS_HOST_READ_BIT,
        srcQueueFamilyIndex:VK_QUEUE_FAMILY_IGNORED,dstQueueFamilyIndex:VK_QUEUE_FAMILY_IGNORED,buffer:s.output,size:FRAME_BYTES as u64,..zero()};
    vkCmdPipelineBarrier(command,VK_PIPELINE_STAGE_TRANSFER_BIT,VK_PIPELINE_STAGE_HOST_BIT,0,0,null(),1,p(&barrier),0,null());
    check!(s,vkEndCommandBuffer(command));
    let submit=VkSubmitInfo{sType:VK_STRUCTURE_TYPE_SUBMIT_INFO,commandBufferCount:1,pCommandBuffers:offset(&command),..zero()};
    check!(s,vkQueueSubmit(s.queue,1,p(&submit),0));check!(s,vkDeviceWaitIdle(s.device));
    check!(s,vkMapMemory(s.device,s.output_mem,0,FRAME_BYTES as u64,0,&mut mapped));
    readMapped(mapped,addr_of_mut!(READBACK).cast(),FRAME_BYTES);vkUnmapMemory(s.device,s.output_mem);Ok(())
}
fn size_of_val_shader<const N:usize>(_:&Shader<N>) -> u32 {N as u32}

#[no_mangle]
pub unsafe extern "C" fn render(require_gpu:u32) -> i32 { render_inner(&mut *addr_of_mut!(STATE),require_gpu!=0).err().unwrap_or(0) }
#[no_mangle]
pub unsafe extern "C" fn cleanup() -> i32 {
    let s=&mut *addr_of_mut!(STATE);let mut result=0;
    if s.device!=0 {
        result=vkDeviceWaitIdle(s.device);
        macro_rules! destroy {($fun:ident,$field:ident)=>{if s.$field!=0 {$fun(s.device,s.$field,null());}};}
        destroy!(vkDestroyCommandPool,pool);destroy!(vkDestroyDescriptorPool,descriptor_pool);destroy!(vkDestroyPipeline,pipeline);
        destroy!(vkDestroyPipelineLayout,pipeline_layout);destroy!(vkDestroyDescriptorSetLayout,set_layout);
        destroy!(vkDestroyShaderModule,vert);destroy!(vkDestroyShaderModule,frag);destroy!(vkDestroyFramebuffer,framebuffer);
        destroy!(vkDestroyRenderPass,pass);destroy!(vkDestroyImageView,view);destroy!(vkDestroyImage,image);
        destroy!(vkFreeMemory,image_mem);destroy!(vkDestroyBuffer,upload);destroy!(vkDestroyBuffer,output);
        destroy!(vkFreeMemory,upload_mem);destroy!(vkFreeMemory,output_mem);vkDestroyDevice(s.device,null());
    }
    if s.instance!=0 {vkDestroyInstance(s.instance,null());}
    *s=zero();result
}
#[no_mangle] pub extern "C" fn frame() -> u32 {addr_of!(READBACK) as usize as u32}
#[no_mangle] pub extern "C" fn linear_frame() -> u32 {addr_of!(tracer::PIXELS) as usize as u32}
#[no_mangle] pub extern "C" fn surface_ids() -> u32 {addr_of!(tracer::SURFACES) as usize as u32}
#[no_mangle] pub extern "C" fn reflection_ids() -> u32 {addr_of!(tracer::REFLECTIONS) as usize as u32}
#[no_mangle] pub unsafe extern "C" fn device_name() -> u32 {addr_of!(STATE.properties.deviceName) as usize as u32}
#[no_mangle] pub unsafe extern "C" fn device_type() -> u32 {STATE.properties.deviceType}
#[no_mangle] pub unsafe extern "C" fn error_line() -> u32 {STATE.error_line}

// Keep the wire ABI's eight-byte handle and size requirements visible in this guest.
const _: ()=assert!(size_of::<u64>()==8);
