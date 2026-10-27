#include "imports.h"
#include "tracer.h"
#include "shaders.h"

#define P(x) ((uint32_t)(uintptr_t)(x))
#define CHECK(call) do { int32_t r=(call); if(r!=VK_SUCCESS) { error_line=__LINE__; return r; } } while(0)
#define REQUIRE(test) do { if(!(test)) { error_line=__LINE__; return -100000; } } while(0)
static uint64_t instance, device, queue, upload, upload_mem, output, output_mem;
static uint64_t image, image_mem, view, pass, framebuffer, vert, frag;
static uint64_t set_layout, pipeline_layout, pipeline, descriptor_pool, pool;
static uint32_t error_line;
static wv32_VkPhysicalDeviceProperties properties;
static wv32_VkPhysicalDeviceMemoryProperties memory;
static uint8_t readback[WIDTH*HEIGHT*4];

// No libc or WASI imports are needed by this freestanding guest.
void *memset(void *dst,int value,size_t count) { unsigned char *p=dst; for(size_t i=0;i<count;i++)p[i]=value; return dst; }
void *memcpy(void *dst,const void *src,size_t count) { unsigned char *d=dst; const unsigned char *s=src; for(size_t i=0;i<count;i++)d[i]=s[i]; return dst; }

static int32_t allocate(wv32_VkMemoryRequirements req,uint32_t flags,uint64_t *out) {
    for(uint32_t i=0;i<memory.memoryTypeCount;i++) {
        if((req.memoryTypeBits&(1u<<i)) && (memory.memoryTypes[i].propertyFlags&flags)==flags) {
            wv32_VkMemoryAllocateInfo a={.sType=VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,.allocationSize=req.size,.memoryTypeIndex=i};
            CHECK(vkAllocateMemory(device,&a,0,out)); return 0;
        }
    }
    REQUIRE(0);
}

static int32_t buffer(uint64_t bytes,uint32_t usage,uint64_t *out,uint64_t *mem) {
    wv32_VkBufferCreateInfo b={.sType=VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,.size=bytes,.usage=usage};
    CHECK(vkCreateBuffer(device,&b,0,out));
    wv32_VkMemoryRequirements req; vkGetBufferMemoryRequirements(device,*out,&req);
    CHECK(allocate(req,VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT|VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,mem));
    CHECK(vkBindBufferMemory(device,*out,*mem,0)); return 0;
}

__attribute__((export_name("render"))) int32_t render(uint32_t require_gpu) {
    REQUIRE(abiVersion()==1);
    wv32_VkApplicationInfo app={.sType=VK_STRUCTURE_TYPE_APPLICATION_INFO,.pApplicationName=P("Wago postcard"),.apiVersion=VK_API_VERSION_1_0};
    wv32_VkInstanceCreateInfo ic={.sType=VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,.pApplicationInfo=P(&app)};
    CHECK(vkCreateInstance(&ic,0,&instance));
    uint64_t devices[16]; uint32_t count=16;
    CHECK(vkEnumeratePhysicalDevices(instance,&count,devices)); REQUIRE(count>0);
    uint64_t physical=0; uint32_t family=0; int best=-1;
    for(uint32_t i=0;i<count;i++) {
        wv32_VkPhysicalDeviceProperties p; vkGetPhysicalDeviceProperties(devices[i],&p);
        if(require_gpu && p.deviceType!=VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU && p.deviceType!=VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU)continue;
        uint32_t n=0; vkGetPhysicalDeviceQueueFamilyProperties(devices[i],&n,0); REQUIRE(n<=64);
        wv32_VkQueueFamilyProperties families[64]; vkGetPhysicalDeviceQueueFamilyProperties(devices[i],&n,families);
        for(uint32_t j=0;j<n;j++)if(families[j].queueCount && (families[j].queueFlags&VK_QUEUE_GRAPHICS_BIT)) {
            int score=p.deviceType==VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU?3:p.deviceType==VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU?2:1;
            if(score>best) { best=score; physical=devices[i]; properties=p; family=j; } break;
        }
    }
    REQUIRE(physical!=0);
    vkGetPhysicalDeviceMemoryProperties(physical,&memory);
    float priority=1;
    wv32_VkDeviceQueueCreateInfo qi={.sType=VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,.queueFamilyIndex=family,.queueCount=1,.pQueuePriorities=P(&priority)};
    wv32_VkDeviceCreateInfo dc={.sType=VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,.queueCreateInfoCount=1,.pQueueCreateInfos=P(&qi)};
    CHECK(vkCreateDevice(physical,&dc,0,&device)); vkGetDeviceQueue(device,family,0,&queue);

    trace_scene();
    CHECK(buffer(sizeof(pixels),VK_BUFFER_USAGE_STORAGE_BUFFER_BIT,&upload,&upload_mem));
    CHECK(buffer(sizeof(readback),VK_BUFFER_USAGE_TRANSFER_DST_BIT,&output,&output_mem));
    uint64_t mapped=0; CHECK(vkMapMemory(device,upload_mem,0,sizeof(pixels),0,&mapped));
    writeMapped(mapped,pixels,sizeof(pixels)); vkUnmapMemory(device,upload_mem);

    wv32_VkImageCreateInfo im={.sType=VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,.imageType=VK_IMAGE_TYPE_2D,.format=VK_FORMAT_R8G8B8A8_UNORM,
        .extent={WIDTH,HEIGHT,1},.mipLevels=1,.arrayLayers=1,.samples=VK_SAMPLE_COUNT_1_BIT,.tiling=VK_IMAGE_TILING_OPTIMAL,
        .usage=VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT|VK_IMAGE_USAGE_TRANSFER_SRC_BIT};
    CHECK(vkCreateImage(device,&im,0,&image));
    wv32_VkMemoryRequirements req; vkGetImageMemoryRequirements(device,image,&req);
    CHECK(allocate(req,VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT,&image_mem)); CHECK(vkBindImageMemory(device,image,image_mem,0));
    wv32_VkImageViewCreateInfo iv={.sType=VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,.image=image,.viewType=VK_IMAGE_VIEW_TYPE_2D,
        .format=im.format,.subresourceRange={VK_IMAGE_ASPECT_COLOR_BIT,0,1,0,1}};
    CHECK(vkCreateImageView(device,&iv,0,&view));
    wv32_VkAttachmentDescription attachment={.format=im.format,.samples=VK_SAMPLE_COUNT_1_BIT,
        .loadOp=VK_ATTACHMENT_LOAD_OP_DONT_CARE,.storeOp=VK_ATTACHMENT_STORE_OP_STORE,
        .stencilLoadOp=VK_ATTACHMENT_LOAD_OP_DONT_CARE,.stencilStoreOp=VK_ATTACHMENT_STORE_OP_DONT_CARE,
        .initialLayout=VK_IMAGE_LAYOUT_UNDEFINED,.finalLayout=VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL};
    wv32_VkAttachmentReference ref={0,VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL};
    wv32_VkSubpassDescription subpass={.pipelineBindPoint=VK_PIPELINE_BIND_POINT_GRAPHICS,.colorAttachmentCount=1,.pColorAttachments=P(&ref)};
    wv32_VkSubpassDependency dependency={.srcSubpass=0,.dstSubpass=VK_SUBPASS_EXTERNAL,
        .srcStageMask=VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,.dstStageMask=VK_PIPELINE_STAGE_TRANSFER_BIT,
        .srcAccessMask=VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,.dstAccessMask=VK_ACCESS_TRANSFER_READ_BIT};
    wv32_VkRenderPassCreateInfo rp={.sType=VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,.attachmentCount=1,.pAttachments=P(&attachment),
        .subpassCount=1,.pSubpasses=P(&subpass),.dependencyCount=1,.pDependencies=P(&dependency)};
    CHECK(vkCreateRenderPass(device,&rp,0,&pass));
    wv32_VkFramebufferCreateInfo fb={.sType=VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,.renderPass=pass,.attachmentCount=1,
        .pAttachments=P(&view),.width=WIDTH,.height=HEIGHT,.layers=1};
    CHECK(vkCreateFramebuffer(device,&fb,0,&framebuffer));

    wv32_VkShaderModuleCreateInfo sm={.sType=VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,.codeSize=sizeof(vertex_spv),.pCode=P(vertex_spv)};
    CHECK(vkCreateShaderModule(device,&sm,0,&vert)); sm.codeSize=sizeof(fragment_spv); sm.pCode=P(fragment_spv);
    CHECK(vkCreateShaderModule(device,&sm,0,&frag));
    wv32_VkDescriptorSetLayoutBinding binding={.binding=0,.descriptorType=VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,.descriptorCount=1,.stageFlags=VK_SHADER_STAGE_FRAGMENT_BIT};
    wv32_VkDescriptorSetLayoutCreateInfo dl={.sType=VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,.bindingCount=1,.pBindings=P(&binding)};
    CHECK(vkCreateDescriptorSetLayout(device,&dl,0,&set_layout));
    wv32_VkPipelineLayoutCreateInfo pl={.sType=VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,.setLayoutCount=1,.pSetLayouts=P(&set_layout)};
    CHECK(vkCreatePipelineLayout(device,&pl,0,&pipeline_layout));
    wv32_VkPipelineShaderStageCreateInfo stages[2]={
        {.sType=VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,.stage=VK_SHADER_STAGE_VERTEX_BIT,.module=vert,.pName=P("main")},
        {.sType=VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,.stage=VK_SHADER_STAGE_FRAGMENT_BIT,.module=frag,.pName=P("main")}};
    wv32_VkPipelineVertexInputStateCreateInfo vi={.sType=VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO};
    wv32_VkPipelineInputAssemblyStateCreateInfo ia={.sType=VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,.topology=VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST};
    wv32_VkViewport viewport={0,0,WIDTH,HEIGHT,0,1}; wv32_VkRect2D scissor={{0,0},{WIDTH,HEIGHT}};
    wv32_VkPipelineViewportStateCreateInfo vp={.sType=VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,.viewportCount=1,.pViewports=P(&viewport),.scissorCount=1,.pScissors=P(&scissor)};
    wv32_VkPipelineRasterizationStateCreateInfo rs={.sType=VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,.lineWidth=1};
    wv32_VkPipelineMultisampleStateCreateInfo ms={.sType=VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,.rasterizationSamples=VK_SAMPLE_COUNT_1_BIT};
    wv32_VkPipelineColorBlendAttachmentState blend={.colorWriteMask=15};
    wv32_VkPipelineColorBlendStateCreateInfo cb={.sType=VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,.attachmentCount=1,.pAttachments=P(&blend)};
    wv32_VkGraphicsPipelineCreateInfo gp={.sType=VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,.stageCount=2,.pStages=P(stages),
        .pVertexInputState=P(&vi),.pInputAssemblyState=P(&ia),.pViewportState=P(&vp),.pRasterizationState=P(&rs),.pMultisampleState=P(&ms),
        .pColorBlendState=P(&cb),.layout=pipeline_layout,.renderPass=pass,.basePipelineIndex=(uint32_t)-1};
    CHECK(vkCreateGraphicsPipelines(device,0,1,&gp,0,&pipeline));
    wv32_VkDescriptorPoolSize size={VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,1};
    wv32_VkDescriptorPoolCreateInfo dp={.sType=VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,.maxSets=1,.poolSizeCount=1,.pPoolSizes=P(&size)};
    CHECK(vkCreateDescriptorPool(device,&dp,0,&descriptor_pool));
    wv32_VkDescriptorSetAllocateInfo ds={.sType=VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,.descriptorPool=descriptor_pool,.descriptorSetCount=1,.pSetLayouts=P(&set_layout)};
    uint64_t set=0; CHECK(vkAllocateDescriptorSets(device,&ds,&set));
    wv32_VkDescriptorBufferInfo db={.buffer=upload,.range=sizeof(pixels)};
    wv32_VkWriteDescriptorSet write={.sType=VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,.dstSet=set,.descriptorCount=1,.descriptorType=VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,.pBufferInfo=P(&db)};
    vkUpdateDescriptorSets(device,1,&write,0,0);

    wv32_VkCommandPoolCreateInfo cp={.sType=VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,.queueFamilyIndex=family};
    CHECK(vkCreateCommandPool(device,&cp,0,&pool));
    wv32_VkCommandBufferAllocateInfo ca={.sType=VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,.commandPool=pool,.commandBufferCount=1};
    uint64_t command=0; CHECK(vkAllocateCommandBuffers(device,&ca,&command));
    wv32_VkCommandBufferBeginInfo begin={.sType=VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,.flags=VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT};
    CHECK(vkBeginCommandBuffer(command,&begin));
    wv32_VkRenderPassBeginInfo rb={.sType=VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,.renderPass=pass,.framebuffer=framebuffer,.renderArea={{0,0},{WIDTH,HEIGHT}}};
    vkCmdBeginRenderPass(command,&rb,VK_SUBPASS_CONTENTS_INLINE);
    vkCmdBindPipeline(command,VK_PIPELINE_BIND_POINT_GRAPHICS,pipeline);
    vkCmdBindDescriptorSets(command,VK_PIPELINE_BIND_POINT_GRAPHICS,pipeline_layout,0,1,&set,0,0);
    vkCmdDraw(command,3,1,0,0); vkCmdEndRenderPass(command);
    wv32_VkBufferImageCopy copy={.imageSubresource={VK_IMAGE_ASPECT_COLOR_BIT,0,0,1},.imageExtent={WIDTH,HEIGHT,1}};
    vkCmdCopyImageToBuffer(command,image,VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,output,1,&copy);
    wv32_VkBufferMemoryBarrier barrier={.sType=VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,.srcAccessMask=VK_ACCESS_TRANSFER_WRITE_BIT,
        .dstAccessMask=VK_ACCESS_HOST_READ_BIT,.srcQueueFamilyIndex=VK_QUEUE_FAMILY_IGNORED,.dstQueueFamilyIndex=VK_QUEUE_FAMILY_IGNORED,.buffer=output,.size=sizeof(readback)};
    vkCmdPipelineBarrier(command,VK_PIPELINE_STAGE_TRANSFER_BIT,VK_PIPELINE_STAGE_HOST_BIT,0,0,0,1,&barrier,0,0);
    CHECK(vkEndCommandBuffer(command));
    wv32_VkSubmitInfo submit={.sType=VK_STRUCTURE_TYPE_SUBMIT_INFO,.commandBufferCount=1,.pCommandBuffers=P(&command)};
    CHECK(vkQueueSubmit(queue,1,&submit,0)); CHECK(vkDeviceWaitIdle(device));
    CHECK(vkMapMemory(device,output_mem,0,sizeof(readback),0,&mapped));
    readMapped(mapped,readback,sizeof(readback)); vkUnmapMemory(device,output_mem);
    return 0;
}

__attribute__((export_name("cleanup"))) int32_t cleanup(void) {
    int32_t result=0;
    if(device) {
        result=vkDeviceWaitIdle(device);
        if(pool)vkDestroyCommandPool(device,pool,0);
        if(descriptor_pool)vkDestroyDescriptorPool(device,descriptor_pool,0);
        if(pipeline)vkDestroyPipeline(device,pipeline,0);
        if(pipeline_layout)vkDestroyPipelineLayout(device,pipeline_layout,0);
        if(set_layout)vkDestroyDescriptorSetLayout(device,set_layout,0);
        if(vert)vkDestroyShaderModule(device,vert,0);
        if(frag)vkDestroyShaderModule(device,frag,0);
        if(framebuffer)vkDestroyFramebuffer(device,framebuffer,0);
        if(pass)vkDestroyRenderPass(device,pass,0);
        if(view)vkDestroyImageView(device,view,0);
        if(image)vkDestroyImage(device,image,0);
        if(image_mem)vkFreeMemory(device,image_mem,0);
        if(upload)vkDestroyBuffer(device,upload,0);
        if(output)vkDestroyBuffer(device,output,0);
        if(upload_mem)vkFreeMemory(device,upload_mem,0);
        if(output_mem)vkFreeMemory(device,output_mem,0);
        vkDestroyDevice(device,0);
    }
    if(instance)vkDestroyInstance(instance,0);
    return result;
}
__attribute__((export_name("frame"))) uint32_t frame(void) { return P(readback); }
__attribute__((export_name("linear_frame"))) uint32_t linear_frame(void) { return P(pixels); }
__attribute__((export_name("surface_ids"))) uint32_t surface_ids(void) { return P(surfaces); }
__attribute__((export_name("reflection_ids"))) uint32_t reflection_ids(void) { return P(reflections); }
__attribute__((export_name("device_name"))) uint32_t device_name(void) { return P(properties.deviceName); }
__attribute__((export_name("device_type"))) uint32_t device_type(void) { return properties.deviceType; }
__attribute__((export_name("error_line"))) uint32_t get_error_line(void) { return error_line; }
