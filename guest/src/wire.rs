// Generated subset of wago-vulkan a3943c4 abi/wire.h. See vendor/ for license.
#![allow(non_snake_case)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkApplicationInfo {
    pub sType: u32,
    pub pNext: u32,
    pub pApplicationName: u32,
    pub applicationVersion: u32,
    pub pEngineName: u32,
    pub engineVersion: u32,
    pub apiVersion: u32,
}
const _: () = assert!(core::mem::size_of::<VkApplicationInfo>() == 28);
const _: () = assert!(core::mem::offset_of!(VkApplicationInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkApplicationInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkApplicationInfo, pApplicationName) == 8);
const _: () = assert!(core::mem::offset_of!(VkApplicationInfo, applicationVersion) == 12);
const _: () = assert!(core::mem::offset_of!(VkApplicationInfo, pEngineName) == 16);
const _: () = assert!(core::mem::offset_of!(VkApplicationInfo, engineVersion) == 20);
const _: () = assert!(core::mem::offset_of!(VkApplicationInfo, apiVersion) == 24);
const _: () = assert!(sizeof(wv64_VkApplicationInfo) == 48);
const _: () = assert!(offsetof(wv64_VkApplicationInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkApplicationInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkApplicationInfo, pApplicationName) == 16);
const _: () = assert!(offsetof(wv64_VkApplicationInfo, applicationVersion) == 24);
const _: () = assert!(offsetof(wv64_VkApplicationInfo, pEngineName) == 32);
const _: () = assert!(offsetof(wv64_VkApplicationInfo, engineVersion) == 40);
const _: () = assert!(offsetof(wv64_VkApplicationInfo, apiVersion) == 44);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAttachmentDescription {
    pub flags: u32,
    pub format: u32,
    pub samples: u32,
    pub loadOp: u32,
    pub storeOp: u32,
    pub stencilLoadOp: u32,
    pub stencilStoreOp: u32,
    pub initialLayout: u32,
    pub finalLayout: u32,
}
const _: () = assert!(core::mem::size_of::<VkAttachmentDescription>() == 36);
const _: () = assert!(core::mem::offset_of!(VkAttachmentDescription, flags) == 0);
const _: () = assert!(core::mem::offset_of!(VkAttachmentDescription, format) == 4);
const _: () = assert!(core::mem::offset_of!(VkAttachmentDescription, samples) == 8);
const _: () = assert!(core::mem::offset_of!(VkAttachmentDescription, loadOp) == 12);
const _: () = assert!(core::mem::offset_of!(VkAttachmentDescription, storeOp) == 16);
const _: () = assert!(core::mem::offset_of!(VkAttachmentDescription, stencilLoadOp) == 20);
const _: () = assert!(core::mem::offset_of!(VkAttachmentDescription, stencilStoreOp) == 24);
const _: () = assert!(core::mem::offset_of!(VkAttachmentDescription, initialLayout) == 28);
const _: () = assert!(core::mem::offset_of!(VkAttachmentDescription, finalLayout) == 32);
const _: () = assert!(sizeof(wv64_VkAttachmentDescription) == 36);
const _: () = assert!(offsetof(wv64_VkAttachmentDescription, flags) == 0);
const _: () = assert!(offsetof(wv64_VkAttachmentDescription, format) == 4);
const _: () = assert!(offsetof(wv64_VkAttachmentDescription, samples) == 8);
const _: () = assert!(offsetof(wv64_VkAttachmentDescription, loadOp) == 12);
const _: () = assert!(offsetof(wv64_VkAttachmentDescription, storeOp) == 16);
const _: () = assert!(offsetof(wv64_VkAttachmentDescription, stencilLoadOp) == 20);
const _: () = assert!(offsetof(wv64_VkAttachmentDescription, stencilStoreOp) == 24);
const _: () = assert!(offsetof(wv64_VkAttachmentDescription, initialLayout) == 28);
const _: () = assert!(offsetof(wv64_VkAttachmentDescription, finalLayout) == 32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: u32,
}
const _: () = assert!(core::mem::size_of::<VkAttachmentReference>() == 8);
const _: () = assert!(core::mem::offset_of!(VkAttachmentReference, attachment) == 0);
const _: () = assert!(core::mem::offset_of!(VkAttachmentReference, layout) == 4);
const _: () = assert!(sizeof(wv64_VkAttachmentReference) == 8);
const _: () = assert!(offsetof(wv64_VkAttachmentReference, attachment) == 0);
const _: () = assert!(offsetof(wv64_VkAttachmentReference, layout) == 4);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub size: u64,
    pub usage: u32,
    pub sharingMode: u32,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: u32,
}
const _: () = assert!(core::mem::size_of::<VkBufferCreateInfo>() == 40);
const _: () = assert!(core::mem::offset_of!(VkBufferCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkBufferCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkBufferCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkBufferCreateInfo, size) == 16);
const _: () = assert!(core::mem::offset_of!(VkBufferCreateInfo, usage) == 24);
const _: () = assert!(core::mem::offset_of!(VkBufferCreateInfo, sharingMode) == 28);
const _: () = assert!(core::mem::offset_of!(VkBufferCreateInfo, queueFamilyIndexCount) == 32);
const _: () = assert!(core::mem::offset_of!(VkBufferCreateInfo, pQueueFamilyIndices) == 36);
const _: () = assert!(sizeof(wv64_VkBufferCreateInfo) == 56);
const _: () = assert!(offsetof(wv64_VkBufferCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkBufferCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkBufferCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkBufferCreateInfo, size) == 24);
const _: () = assert!(offsetof(wv64_VkBufferCreateInfo, usage) == 32);
const _: () = assert!(offsetof(wv64_VkBufferCreateInfo, sharingMode) == 36);
const _: () = assert!(offsetof(wv64_VkBufferCreateInfo, queueFamilyIndexCount) == 40);
const _: () = assert!(offsetof(wv64_VkBufferCreateInfo, pQueueFamilyIndices) == 48);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferImageCopy {
    pub bufferOffset: u64,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}
const _: () = assert!(core::mem::size_of::<VkBufferImageCopy>() == 56);
const _: () = assert!(core::mem::offset_of!(VkBufferImageCopy, bufferOffset) == 0);
const _: () = assert!(core::mem::offset_of!(VkBufferImageCopy, bufferRowLength) == 8);
const _: () = assert!(core::mem::offset_of!(VkBufferImageCopy, bufferImageHeight) == 12);
const _: () = assert!(core::mem::offset_of!(VkBufferImageCopy, imageSubresource) == 16);
const _: () = assert!(core::mem::offset_of!(VkBufferImageCopy, imageOffset) == 32);
const _: () = assert!(core::mem::offset_of!(VkBufferImageCopy, imageExtent) == 44);
const _: () = assert!(sizeof(wv64_VkBufferImageCopy) == 56);
const _: () = assert!(offsetof(wv64_VkBufferImageCopy, bufferOffset) == 0);
const _: () = assert!(offsetof(wv64_VkBufferImageCopy, bufferRowLength) == 8);
const _: () = assert!(offsetof(wv64_VkBufferImageCopy, bufferImageHeight) == 12);
const _: () = assert!(offsetof(wv64_VkBufferImageCopy, imageSubresource) == 16);
const _: () = assert!(offsetof(wv64_VkBufferImageCopy, imageOffset) == 32);
const _: () = assert!(offsetof(wv64_VkBufferImageCopy, imageExtent) == 44);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferMemoryBarrier {
    pub sType: u32,
    pub pNext: u32,
    pub srcAccessMask: u32,
    pub dstAccessMask: u32,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: u64,
    pub offset: u64,
    pub size: u64,
}
const _: () = assert!(core::mem::size_of::<VkBufferMemoryBarrier>() == 48);
const _: () = assert!(core::mem::offset_of!(VkBufferMemoryBarrier, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkBufferMemoryBarrier, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkBufferMemoryBarrier, srcAccessMask) == 8);
const _: () = assert!(core::mem::offset_of!(VkBufferMemoryBarrier, dstAccessMask) == 12);
const _: () = assert!(core::mem::offset_of!(VkBufferMemoryBarrier, srcQueueFamilyIndex) == 16);
const _: () = assert!(core::mem::offset_of!(VkBufferMemoryBarrier, dstQueueFamilyIndex) == 20);
const _: () = assert!(core::mem::offset_of!(VkBufferMemoryBarrier, buffer) == 24);
const _: () = assert!(core::mem::offset_of!(VkBufferMemoryBarrier, offset) == 32);
const _: () = assert!(core::mem::offset_of!(VkBufferMemoryBarrier, size) == 40);
const _: () = assert!(sizeof(wv64_VkBufferMemoryBarrier) == 56);
const _: () = assert!(offsetof(wv64_VkBufferMemoryBarrier, sType) == 0);
const _: () = assert!(offsetof(wv64_VkBufferMemoryBarrier, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkBufferMemoryBarrier, srcAccessMask) == 16);
const _: () = assert!(offsetof(wv64_VkBufferMemoryBarrier, dstAccessMask) == 20);
const _: () = assert!(offsetof(wv64_VkBufferMemoryBarrier, srcQueueFamilyIndex) == 24);
const _: () = assert!(offsetof(wv64_VkBufferMemoryBarrier, dstQueueFamilyIndex) == 28);
const _: () = assert!(offsetof(wv64_VkBufferMemoryBarrier, buffer) == 32);
const _: () = assert!(offsetof(wv64_VkBufferMemoryBarrier, offset) == 40);
const _: () = assert!(offsetof(wv64_VkBufferMemoryBarrier, size) == 48);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub commandPool: u64,
    pub level: u32,
    pub commandBufferCount: u32,
}
const _: () = assert!(core::mem::size_of::<VkCommandBufferAllocateInfo>() == 24);
const _: () = assert!(core::mem::offset_of!(VkCommandBufferAllocateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkCommandBufferAllocateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkCommandBufferAllocateInfo, commandPool) == 8);
const _: () = assert!(core::mem::offset_of!(VkCommandBufferAllocateInfo, level) == 16);
const _: () = assert!(core::mem::offset_of!(VkCommandBufferAllocateInfo, commandBufferCount) == 20);
const _: () = assert!(sizeof(wv64_VkCommandBufferAllocateInfo) == 32);
const _: () = assert!(offsetof(wv64_VkCommandBufferAllocateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkCommandBufferAllocateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkCommandBufferAllocateInfo, commandPool) == 16);
const _: () = assert!(offsetof(wv64_VkCommandBufferAllocateInfo, level) == 24);
const _: () = assert!(offsetof(wv64_VkCommandBufferAllocateInfo, commandBufferCount) == 28);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCommandBufferBeginInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub pInheritanceInfo: u32,
}
const _: () = assert!(core::mem::size_of::<VkCommandBufferBeginInfo>() == 16);
const _: () = assert!(core::mem::offset_of!(VkCommandBufferBeginInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkCommandBufferBeginInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkCommandBufferBeginInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkCommandBufferBeginInfo, pInheritanceInfo) == 12);
const _: () = assert!(sizeof(wv64_VkCommandBufferBeginInfo) == 32);
const _: () = assert!(offsetof(wv64_VkCommandBufferBeginInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkCommandBufferBeginInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkCommandBufferBeginInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkCommandBufferBeginInfo, pInheritanceInfo) == 24);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCommandPoolCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub queueFamilyIndex: u32,
}
const _: () = assert!(core::mem::size_of::<VkCommandPoolCreateInfo>() == 16);
const _: () = assert!(core::mem::offset_of!(VkCommandPoolCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkCommandPoolCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkCommandPoolCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkCommandPoolCreateInfo, queueFamilyIndex) == 12);
const _: () = assert!(sizeof(wv64_VkCommandPoolCreateInfo) == 24);
const _: () = assert!(offsetof(wv64_VkCommandPoolCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkCommandPoolCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkCommandPoolCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkCommandPoolCreateInfo, queueFamilyIndex) == 20);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkComponentMapping {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}
const _: () = assert!(core::mem::size_of::<VkComponentMapping>() == 16);
const _: () = assert!(core::mem::offset_of!(VkComponentMapping, r) == 0);
const _: () = assert!(core::mem::offset_of!(VkComponentMapping, g) == 4);
const _: () = assert!(core::mem::offset_of!(VkComponentMapping, b) == 8);
const _: () = assert!(core::mem::offset_of!(VkComponentMapping, a) == 12);
const _: () = assert!(sizeof(wv64_VkComponentMapping) == 16);
const _: () = assert!(offsetof(wv64_VkComponentMapping, r) == 0);
const _: () = assert!(offsetof(wv64_VkComponentMapping, g) == 4);
const _: () = assert!(offsetof(wv64_VkComponentMapping, b) == 8);
const _: () = assert!(offsetof(wv64_VkComponentMapping, a) == 12);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorBufferInfo {
    pub buffer: u64,
    pub offset: u64,
    pub range: u64,
}
const _: () = assert!(core::mem::size_of::<VkDescriptorBufferInfo>() == 24);
const _: () = assert!(core::mem::offset_of!(VkDescriptorBufferInfo, buffer) == 0);
const _: () = assert!(core::mem::offset_of!(VkDescriptorBufferInfo, offset) == 8);
const _: () = assert!(core::mem::offset_of!(VkDescriptorBufferInfo, range) == 16);
const _: () = assert!(sizeof(wv64_VkDescriptorBufferInfo) == 24);
const _: () = assert!(offsetof(wv64_VkDescriptorBufferInfo, buffer) == 0);
const _: () = assert!(offsetof(wv64_VkDescriptorBufferInfo, offset) == 8);
const _: () = assert!(offsetof(wv64_VkDescriptorBufferInfo, range) == 16);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorPoolCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: u32,
}
const _: () = assert!(core::mem::size_of::<VkDescriptorPoolCreateInfo>() == 24);
const _: () = assert!(core::mem::offset_of!(VkDescriptorPoolCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkDescriptorPoolCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkDescriptorPoolCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkDescriptorPoolCreateInfo, maxSets) == 12);
const _: () = assert!(core::mem::offset_of!(VkDescriptorPoolCreateInfo, poolSizeCount) == 16);
const _: () = assert!(core::mem::offset_of!(VkDescriptorPoolCreateInfo, pPoolSizes) == 20);
const _: () = assert!(sizeof(wv64_VkDescriptorPoolCreateInfo) == 40);
const _: () = assert!(offsetof(wv64_VkDescriptorPoolCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkDescriptorPoolCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkDescriptorPoolCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkDescriptorPoolCreateInfo, maxSets) == 20);
const _: () = assert!(offsetof(wv64_VkDescriptorPoolCreateInfo, poolSizeCount) == 24);
const _: () = assert!(offsetof(wv64_VkDescriptorPoolCreateInfo, pPoolSizes) == 32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorPoolSize {
    pub type: u32,
    pub descriptorCount: u32,
}
const _: () = assert!(core::mem::size_of::<VkDescriptorPoolSize>() == 8);
const _: () = assert!(core::mem::offset_of!(VkDescriptorPoolSize, type) == 0);
const _: () = assert!(core::mem::offset_of!(VkDescriptorPoolSize, descriptorCount) == 4);
const _: () = assert!(sizeof(wv64_VkDescriptorPoolSize) == 8);
const _: () = assert!(offsetof(wv64_VkDescriptorPoolSize, type) == 0);
const _: () = assert!(offsetof(wv64_VkDescriptorPoolSize, descriptorCount) == 4);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorSetAllocateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub descriptorPool: u64,
    pub descriptorSetCount: u32,
    pub pSetLayouts: u32,
}
const _: () = assert!(core::mem::size_of::<VkDescriptorSetAllocateInfo>() == 24);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetAllocateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetAllocateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetAllocateInfo, descriptorPool) == 8);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetAllocateInfo, descriptorSetCount) == 16);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetAllocateInfo, pSetLayouts) == 20);
const _: () = assert!(sizeof(wv64_VkDescriptorSetAllocateInfo) == 40);
const _: () = assert!(offsetof(wv64_VkDescriptorSetAllocateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkDescriptorSetAllocateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkDescriptorSetAllocateInfo, descriptorPool) == 16);
const _: () = assert!(offsetof(wv64_VkDescriptorSetAllocateInfo, descriptorSetCount) == 24);
const _: () = assert!(offsetof(wv64_VkDescriptorSetAllocateInfo, pSetLayouts) == 32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: u32,
    pub descriptorCount: u32,
    pub stageFlags: u32,
    pub pImmutableSamplers: u32,
}
const _: () = assert!(core::mem::size_of::<VkDescriptorSetLayoutBinding>() == 20);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetLayoutBinding, binding) == 0);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetLayoutBinding, descriptorType) == 4);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetLayoutBinding, descriptorCount) == 8);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetLayoutBinding, stageFlags) == 12);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetLayoutBinding, pImmutableSamplers) == 16);
const _: () = assert!(sizeof(wv64_VkDescriptorSetLayoutBinding) == 24);
const _: () = assert!(offsetof(wv64_VkDescriptorSetLayoutBinding, binding) == 0);
const _: () = assert!(offsetof(wv64_VkDescriptorSetLayoutBinding, descriptorType) == 4);
const _: () = assert!(offsetof(wv64_VkDescriptorSetLayoutBinding, descriptorCount) == 8);
const _: () = assert!(offsetof(wv64_VkDescriptorSetLayoutBinding, stageFlags) == 12);
const _: () = assert!(offsetof(wv64_VkDescriptorSetLayoutBinding, pImmutableSamplers) == 16);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub bindingCount: u32,
    pub pBindings: u32,
}
const _: () = assert!(core::mem::size_of::<VkDescriptorSetLayoutCreateInfo>() == 20);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetLayoutCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetLayoutCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetLayoutCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetLayoutCreateInfo, bindingCount) == 12);
const _: () = assert!(core::mem::offset_of!(VkDescriptorSetLayoutCreateInfo, pBindings) == 16);
const _: () = assert!(sizeof(wv64_VkDescriptorSetLayoutCreateInfo) == 32);
const _: () = assert!(offsetof(wv64_VkDescriptorSetLayoutCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkDescriptorSetLayoutCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkDescriptorSetLayoutCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkDescriptorSetLayoutCreateInfo, bindingCount) == 20);
const _: () = assert!(offsetof(wv64_VkDescriptorSetLayoutCreateInfo, pBindings) == 24);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: u32,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: u32,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: u32,
    pub pEnabledFeatures: u32,
}
const _: () = assert!(core::mem::size_of::<VkDeviceCreateInfo>() == 40);
const _: () = assert!(core::mem::offset_of!(VkDeviceCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkDeviceCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkDeviceCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkDeviceCreateInfo, queueCreateInfoCount) == 12);
const _: () = assert!(core::mem::offset_of!(VkDeviceCreateInfo, pQueueCreateInfos) == 16);
const _: () = assert!(core::mem::offset_of!(VkDeviceCreateInfo, enabledLayerCount) == 20);
const _: () = assert!(core::mem::offset_of!(VkDeviceCreateInfo, ppEnabledLayerNames) == 24);
const _: () = assert!(core::mem::offset_of!(VkDeviceCreateInfo, enabledExtensionCount) == 28);
const _: () = assert!(core::mem::offset_of!(VkDeviceCreateInfo, ppEnabledExtensionNames) == 32);
const _: () = assert!(core::mem::offset_of!(VkDeviceCreateInfo, pEnabledFeatures) == 36);
const _: () = assert!(sizeof(wv64_VkDeviceCreateInfo) == 72);
const _: () = assert!(offsetof(wv64_VkDeviceCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkDeviceCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkDeviceCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkDeviceCreateInfo, queueCreateInfoCount) == 20);
const _: () = assert!(offsetof(wv64_VkDeviceCreateInfo, pQueueCreateInfos) == 24);
const _: () = assert!(offsetof(wv64_VkDeviceCreateInfo, enabledLayerCount) == 32);
const _: () = assert!(offsetof(wv64_VkDeviceCreateInfo, ppEnabledLayerNames) == 40);
const _: () = assert!(offsetof(wv64_VkDeviceCreateInfo, enabledExtensionCount) == 48);
const _: () = assert!(offsetof(wv64_VkDeviceCreateInfo, ppEnabledExtensionNames) == 56);
const _: () = assert!(offsetof(wv64_VkDeviceCreateInfo, pEnabledFeatures) == 64);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: u32,
}
const _: () = assert!(core::mem::size_of::<VkDeviceQueueCreateInfo>() == 24);
const _: () = assert!(core::mem::offset_of!(VkDeviceQueueCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkDeviceQueueCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkDeviceQueueCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkDeviceQueueCreateInfo, queueFamilyIndex) == 12);
const _: () = assert!(core::mem::offset_of!(VkDeviceQueueCreateInfo, queueCount) == 16);
const _: () = assert!(core::mem::offset_of!(VkDeviceQueueCreateInfo, pQueuePriorities) == 20);
const _: () = assert!(sizeof(wv64_VkDeviceQueueCreateInfo) == 40);
const _: () = assert!(offsetof(wv64_VkDeviceQueueCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkDeviceQueueCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkDeviceQueueCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkDeviceQueueCreateInfo, queueFamilyIndex) == 20);
const _: () = assert!(offsetof(wv64_VkDeviceQueueCreateInfo, queueCount) == 24);
const _: () = assert!(offsetof(wv64_VkDeviceQueueCreateInfo, pQueuePriorities) == 32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}
const _: () = assert!(core::mem::size_of::<VkExtent2D>() == 8);
const _: () = assert!(core::mem::offset_of!(VkExtent2D, width) == 0);
const _: () = assert!(core::mem::offset_of!(VkExtent2D, height) == 4);
const _: () = assert!(sizeof(wv64_VkExtent2D) == 8);
const _: () = assert!(offsetof(wv64_VkExtent2D, width) == 0);
const _: () = assert!(offsetof(wv64_VkExtent2D, height) == 4);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
const _: () = assert!(core::mem::size_of::<VkExtent3D>() == 12);
const _: () = assert!(core::mem::offset_of!(VkExtent3D, width) == 0);
const _: () = assert!(core::mem::offset_of!(VkExtent3D, height) == 4);
const _: () = assert!(core::mem::offset_of!(VkExtent3D, depth) == 8);
const _: () = assert!(sizeof(wv64_VkExtent3D) == 12);
const _: () = assert!(offsetof(wv64_VkExtent3D, width) == 0);
const _: () = assert!(offsetof(wv64_VkExtent3D, height) == 4);
const _: () = assert!(offsetof(wv64_VkExtent3D, depth) == 8);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkFramebufferCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub renderPass: u64,
    pub attachmentCount: u32,
    pub pAttachments: u32,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}
const _: () = assert!(core::mem::size_of::<VkFramebufferCreateInfo>() == 48);
const _: () = assert!(core::mem::offset_of!(VkFramebufferCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkFramebufferCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkFramebufferCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkFramebufferCreateInfo, renderPass) == 16);
const _: () = assert!(core::mem::offset_of!(VkFramebufferCreateInfo, attachmentCount) == 24);
const _: () = assert!(core::mem::offset_of!(VkFramebufferCreateInfo, pAttachments) == 28);
const _: () = assert!(core::mem::offset_of!(VkFramebufferCreateInfo, width) == 32);
const _: () = assert!(core::mem::offset_of!(VkFramebufferCreateInfo, height) == 36);
const _: () = assert!(core::mem::offset_of!(VkFramebufferCreateInfo, layers) == 40);
const _: () = assert!(sizeof(wv64_VkFramebufferCreateInfo) == 64);
const _: () = assert!(offsetof(wv64_VkFramebufferCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkFramebufferCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkFramebufferCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkFramebufferCreateInfo, renderPass) == 24);
const _: () = assert!(offsetof(wv64_VkFramebufferCreateInfo, attachmentCount) == 32);
const _: () = assert!(offsetof(wv64_VkFramebufferCreateInfo, pAttachments) == 40);
const _: () = assert!(offsetof(wv64_VkFramebufferCreateInfo, width) == 48);
const _: () = assert!(offsetof(wv64_VkFramebufferCreateInfo, height) == 52);
const _: () = assert!(offsetof(wv64_VkFramebufferCreateInfo, layers) == 56);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkGraphicsPipelineCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub stageCount: u32,
    pub pStages: u32,
    pub pVertexInputState: u32,
    pub pInputAssemblyState: u32,
    pub pTessellationState: u32,
    pub pViewportState: u32,
    pub pRasterizationState: u32,
    pub pMultisampleState: u32,
    pub pDepthStencilState: u32,
    pub pColorBlendState: u32,
    pub pDynamicState: u32,
    pub layout: u64,
    pub renderPass: u64,
    pub subpass: u32,
    pub basePipelineHandle: u64,
    pub basePipelineIndex: u32,
}
const _: () = assert!(core::mem::size_of::<VkGraphicsPipelineCreateInfo>() == 96);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, stageCount) == 12);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pStages) == 16);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pVertexInputState) == 20);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pInputAssemblyState) == 24);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pTessellationState) == 28);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pViewportState) == 32);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pRasterizationState) == 36);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pMultisampleState) == 40);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pDepthStencilState) == 44);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pColorBlendState) == 48);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, pDynamicState) == 52);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, layout) == 56);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, renderPass) == 64);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, subpass) == 72);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, basePipelineHandle) == 80);
const _: () = assert!(core::mem::offset_of!(VkGraphicsPipelineCreateInfo, basePipelineIndex) == 88);
const _: () = assert!(sizeof(wv64_VkGraphicsPipelineCreateInfo) == 144);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, stageCount) == 20);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pStages) == 24);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pVertexInputState) == 32);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pInputAssemblyState) == 40);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pTessellationState) == 48);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pViewportState) == 56);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pRasterizationState) == 64);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pMultisampleState) == 72);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pDepthStencilState) == 80);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pColorBlendState) == 88);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, pDynamicState) == 96);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, layout) == 104);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, renderPass) == 112);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, subpass) == 120);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, basePipelineHandle) == 128);
const _: () = assert!(offsetof(wv64_VkGraphicsPipelineCreateInfo, basePipelineIndex) == 136);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub imageType: u32,
    pub format: u32,
    pub extent: VkExtent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: u32,
    pub tiling: u32,
    pub usage: u32,
    pub sharingMode: u32,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: u32,
    pub initialLayout: u32,
}
const _: () = assert!(core::mem::size_of::<VkImageCreateInfo>() == 68);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, imageType) == 12);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, format) == 16);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, extent) == 20);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, mipLevels) == 32);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, arrayLayers) == 36);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, samples) == 40);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, tiling) == 44);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, usage) == 48);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, sharingMode) == 52);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, queueFamilyIndexCount) == 56);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, pQueueFamilyIndices) == 60);
const _: () = assert!(core::mem::offset_of!(VkImageCreateInfo, initialLayout) == 64);
const _: () = assert!(sizeof(wv64_VkImageCreateInfo) == 88);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, imageType) == 20);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, format) == 24);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, extent) == 28);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, mipLevels) == 40);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, arrayLayers) == 44);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, samples) == 48);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, tiling) == 52);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, usage) == 56);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, sharingMode) == 60);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, queueFamilyIndexCount) == 64);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, pQueueFamilyIndices) == 72);
const _: () = assert!(offsetof(wv64_VkImageCreateInfo, initialLayout) == 80);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageSubresourceLayers {
    pub aspectMask: u32,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}
const _: () = assert!(core::mem::size_of::<VkImageSubresourceLayers>() == 16);
const _: () = assert!(core::mem::offset_of!(VkImageSubresourceLayers, aspectMask) == 0);
const _: () = assert!(core::mem::offset_of!(VkImageSubresourceLayers, mipLevel) == 4);
const _: () = assert!(core::mem::offset_of!(VkImageSubresourceLayers, baseArrayLayer) == 8);
const _: () = assert!(core::mem::offset_of!(VkImageSubresourceLayers, layerCount) == 12);
const _: () = assert!(sizeof(wv64_VkImageSubresourceLayers) == 16);
const _: () = assert!(offsetof(wv64_VkImageSubresourceLayers, aspectMask) == 0);
const _: () = assert!(offsetof(wv64_VkImageSubresourceLayers, mipLevel) == 4);
const _: () = assert!(offsetof(wv64_VkImageSubresourceLayers, baseArrayLayer) == 8);
const _: () = assert!(offsetof(wv64_VkImageSubresourceLayers, layerCount) == 12);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageSubresourceRange {
    pub aspectMask: u32,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}
const _: () = assert!(core::mem::size_of::<VkImageSubresourceRange>() == 20);
const _: () = assert!(core::mem::offset_of!(VkImageSubresourceRange, aspectMask) == 0);
const _: () = assert!(core::mem::offset_of!(VkImageSubresourceRange, baseMipLevel) == 4);
const _: () = assert!(core::mem::offset_of!(VkImageSubresourceRange, levelCount) == 8);
const _: () = assert!(core::mem::offset_of!(VkImageSubresourceRange, baseArrayLayer) == 12);
const _: () = assert!(core::mem::offset_of!(VkImageSubresourceRange, layerCount) == 16);
const _: () = assert!(sizeof(wv64_VkImageSubresourceRange) == 20);
const _: () = assert!(offsetof(wv64_VkImageSubresourceRange, aspectMask) == 0);
const _: () = assert!(offsetof(wv64_VkImageSubresourceRange, baseMipLevel) == 4);
const _: () = assert!(offsetof(wv64_VkImageSubresourceRange, levelCount) == 8);
const _: () = assert!(offsetof(wv64_VkImageSubresourceRange, baseArrayLayer) == 12);
const _: () = assert!(offsetof(wv64_VkImageSubresourceRange, layerCount) == 16);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageViewCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub image: u64,
    pub viewType: u32,
    pub format: u32,
    pub components: VkComponentMapping,
    pub subresourceRange: VkImageSubresourceRange,
}
const _: () = assert!(core::mem::size_of::<VkImageViewCreateInfo>() == 72);
const _: () = assert!(core::mem::offset_of!(VkImageViewCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkImageViewCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkImageViewCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkImageViewCreateInfo, image) == 16);
const _: () = assert!(core::mem::offset_of!(VkImageViewCreateInfo, viewType) == 24);
const _: () = assert!(core::mem::offset_of!(VkImageViewCreateInfo, format) == 28);
const _: () = assert!(core::mem::offset_of!(VkImageViewCreateInfo, components) == 32);
const _: () = assert!(core::mem::offset_of!(VkImageViewCreateInfo, subresourceRange) == 48);
const _: () = assert!(sizeof(wv64_VkImageViewCreateInfo) == 80);
const _: () = assert!(offsetof(wv64_VkImageViewCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkImageViewCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkImageViewCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkImageViewCreateInfo, image) == 24);
const _: () = assert!(offsetof(wv64_VkImageViewCreateInfo, viewType) == 32);
const _: () = assert!(offsetof(wv64_VkImageViewCreateInfo, format) == 36);
const _: () = assert!(offsetof(wv64_VkImageViewCreateInfo, components) == 40);
const _: () = assert!(offsetof(wv64_VkImageViewCreateInfo, subresourceRange) == 56);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkInstanceCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub pApplicationInfo: u32,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: u32,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: u32,
}
const _: () = assert!(core::mem::size_of::<VkInstanceCreateInfo>() == 32);
const _: () = assert!(core::mem::offset_of!(VkInstanceCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkInstanceCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkInstanceCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkInstanceCreateInfo, pApplicationInfo) == 12);
const _: () = assert!(core::mem::offset_of!(VkInstanceCreateInfo, enabledLayerCount) == 16);
const _: () = assert!(core::mem::offset_of!(VkInstanceCreateInfo, ppEnabledLayerNames) == 20);
const _: () = assert!(core::mem::offset_of!(VkInstanceCreateInfo, enabledExtensionCount) == 24);
const _: () = assert!(core::mem::offset_of!(VkInstanceCreateInfo, ppEnabledExtensionNames) == 28);
const _: () = assert!(sizeof(wv64_VkInstanceCreateInfo) == 64);
const _: () = assert!(offsetof(wv64_VkInstanceCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkInstanceCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkInstanceCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkInstanceCreateInfo, pApplicationInfo) == 24);
const _: () = assert!(offsetof(wv64_VkInstanceCreateInfo, enabledLayerCount) == 32);
const _: () = assert!(offsetof(wv64_VkInstanceCreateInfo, ppEnabledLayerNames) == 40);
const _: () = assert!(offsetof(wv64_VkInstanceCreateInfo, enabledExtensionCount) == 48);
const _: () = assert!(offsetof(wv64_VkInstanceCreateInfo, ppEnabledExtensionNames) == 56);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryAllocateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub allocationSize: u64,
    pub memoryTypeIndex: u32,
}
const _: () = assert!(core::mem::size_of::<VkMemoryAllocateInfo>() == 24);
const _: () = assert!(core::mem::offset_of!(VkMemoryAllocateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkMemoryAllocateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkMemoryAllocateInfo, allocationSize) == 8);
const _: () = assert!(core::mem::offset_of!(VkMemoryAllocateInfo, memoryTypeIndex) == 16);
const _: () = assert!(sizeof(wv64_VkMemoryAllocateInfo) == 32);
const _: () = assert!(offsetof(wv64_VkMemoryAllocateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkMemoryAllocateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkMemoryAllocateInfo, allocationSize) == 16);
const _: () = assert!(offsetof(wv64_VkMemoryAllocateInfo, memoryTypeIndex) == 24);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryHeap {
    pub size: u64,
    pub flags: u32,
}
const _: () = assert!(core::mem::size_of::<VkMemoryHeap>() == 16);
const _: () = assert!(core::mem::offset_of!(VkMemoryHeap, size) == 0);
const _: () = assert!(core::mem::offset_of!(VkMemoryHeap, flags) == 8);
const _: () = assert!(sizeof(wv64_VkMemoryHeap) == 16);
const _: () = assert!(offsetof(wv64_VkMemoryHeap, size) == 0);
const _: () = assert!(offsetof(wv64_VkMemoryHeap, flags) == 8);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryRequirements {
    pub size: u64,
    pub alignment: u64,
    pub memoryTypeBits: u32,
}
const _: () = assert!(core::mem::size_of::<VkMemoryRequirements>() == 24);
const _: () = assert!(core::mem::offset_of!(VkMemoryRequirements, size) == 0);
const _: () = assert!(core::mem::offset_of!(VkMemoryRequirements, alignment) == 8);
const _: () = assert!(core::mem::offset_of!(VkMemoryRequirements, memoryTypeBits) == 16);
const _: () = assert!(sizeof(wv64_VkMemoryRequirements) == 24);
const _: () = assert!(offsetof(wv64_VkMemoryRequirements, size) == 0);
const _: () = assert!(offsetof(wv64_VkMemoryRequirements, alignment) == 8);
const _: () = assert!(offsetof(wv64_VkMemoryRequirements, memoryTypeBits) == 16);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryType {
    pub propertyFlags: u32,
    pub heapIndex: u32,
}
const _: () = assert!(core::mem::size_of::<VkMemoryType>() == 8);
const _: () = assert!(core::mem::offset_of!(VkMemoryType, propertyFlags) == 0);
const _: () = assert!(core::mem::offset_of!(VkMemoryType, heapIndex) == 4);
const _: () = assert!(sizeof(wv64_VkMemoryType) == 8);
const _: () = assert!(offsetof(wv64_VkMemoryType, propertyFlags) == 0);
const _: () = assert!(offsetof(wv64_VkMemoryType, heapIndex) == 4);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkOffset2D {
    pub x: u32,
    pub y: u32,
}
const _: () = assert!(core::mem::size_of::<VkOffset2D>() == 8);
const _: () = assert!(core::mem::offset_of!(VkOffset2D, x) == 0);
const _: () = assert!(core::mem::offset_of!(VkOffset2D, y) == 4);
const _: () = assert!(sizeof(wv64_VkOffset2D) == 8);
const _: () = assert!(offsetof(wv64_VkOffset2D, x) == 0);
const _: () = assert!(offsetof(wv64_VkOffset2D, y) == 4);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkOffset3D {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
const _: () = assert!(core::mem::size_of::<VkOffset3D>() == 12);
const _: () = assert!(core::mem::offset_of!(VkOffset3D, x) == 0);
const _: () = assert!(core::mem::offset_of!(VkOffset3D, y) == 4);
const _: () = assert!(core::mem::offset_of!(VkOffset3D, z) == 8);
const _: () = assert!(sizeof(wv64_VkOffset3D) == 12);
const _: () = assert!(offsetof(wv64_VkOffset3D, x) == 0);
const _: () = assert!(offsetof(wv64_VkOffset3D, y) == 4);
const _: () = assert!(offsetof(wv64_VkOffset3D, z) == 8);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: u64,
    pub sparseAddressSpaceSize: u64,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: f32,
    pub maxSamplerAnisotropy: f32,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [f32; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: u32,
    pub minTexelBufferOffsetAlignment: u64,
    pub minUniformBufferOffsetAlignment: u64,
    pub minStorageBufferOffsetAlignment: u64,
    pub minTexelOffset: u32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: u32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: f32,
    pub maxInterpolationOffset: f32,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: u32,
    pub framebufferDepthSampleCounts: u32,
    pub framebufferStencilSampleCounts: u32,
    pub framebufferNoAttachmentsSampleCounts: u32,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: u32,
    pub sampledImageIntegerSampleCounts: u32,
    pub sampledImageDepthSampleCounts: u32,
    pub sampledImageStencilSampleCounts: u32,
    pub storageImageSampleCounts: u32,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: u32,
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2],
    pub lineWidthRange: [f32; 2],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
    pub strictLines: u32,
    pub standardSampleLocations: u32,
    pub optimalBufferCopyOffsetAlignment: u64,
    pub optimalBufferCopyRowPitchAlignment: u64,
    pub nonCoherentAtomSize: u64,
}
const _: () = assert!(core::mem::size_of::<VkPhysicalDeviceLimits>() == 496);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxImageDimension1D) == 0);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxImageDimension2D) == 4);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxImageDimension3D) == 8);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxImageDimensionCube) == 12);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxImageArrayLayers) == 16);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTexelBufferElements) == 20);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxUniformBufferRange) == 24);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxStorageBufferRange) == 28);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxPushConstantsSize) == 32);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxMemoryAllocationCount) == 36);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxSamplerAllocationCount) == 40);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, bufferImageGranularity) == 48);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, sparseAddressSpaceSize) == 56);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxBoundDescriptorSets) == 64);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxPerStageDescriptorSamplers) == 68);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxPerStageDescriptorUniformBuffers) == 72);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxPerStageDescriptorStorageBuffers) == 76);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxPerStageDescriptorSampledImages) == 80);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxPerStageDescriptorStorageImages) == 84);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxPerStageDescriptorInputAttachments) == 88);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxPerStageResources) == 92);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxDescriptorSetSamplers) == 96);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxDescriptorSetUniformBuffers) == 100);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxDescriptorSetUniformBuffersDynamic) == 104);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxDescriptorSetStorageBuffers) == 108);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxDescriptorSetStorageBuffersDynamic) == 112);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxDescriptorSetSampledImages) == 116);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxDescriptorSetStorageImages) == 120);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxDescriptorSetInputAttachments) == 124);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxVertexInputAttributes) == 128);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxVertexInputBindings) == 132);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxVertexInputAttributeOffset) == 136);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxVertexInputBindingStride) == 140);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxVertexOutputComponents) == 144);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTessellationGenerationLevel) == 148);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTessellationPatchSize) == 152);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTessellationControlPerVertexInputComponents) == 156);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTessellationControlPerVertexOutputComponents) == 160);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTessellationControlPerPatchOutputComponents) == 164);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTessellationControlTotalOutputComponents) == 168);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTessellationEvaluationInputComponents) == 172);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTessellationEvaluationOutputComponents) == 176);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxGeometryShaderInvocations) == 180);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxGeometryInputComponents) == 184);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxGeometryOutputComponents) == 188);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxGeometryOutputVertices) == 192);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxGeometryTotalOutputComponents) == 196);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxFragmentInputComponents) == 200);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxFragmentOutputAttachments) == 204);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxFragmentDualSrcAttachments) == 208);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxFragmentCombinedOutputResources) == 212);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxComputeSharedMemorySize) == 216);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxComputeWorkGroupCount) == 220);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxComputeWorkGroupInvocations) == 232);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxComputeWorkGroupSize) == 236);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, subPixelPrecisionBits) == 248);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, subTexelPrecisionBits) == 252);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, mipmapPrecisionBits) == 256);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxDrawIndexedIndexValue) == 260);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxDrawIndirectCount) == 264);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxSamplerLodBias) == 268);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxSamplerAnisotropy) == 272);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxViewports) == 276);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxViewportDimensions) == 280);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, viewportBoundsRange) == 288);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, viewportSubPixelBits) == 296);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, minMemoryMapAlignment) == 300);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, minTexelBufferOffsetAlignment) == 304);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, minUniformBufferOffsetAlignment) == 312);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, minStorageBufferOffsetAlignment) == 320);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, minTexelOffset) == 328);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTexelOffset) == 332);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, minTexelGatherOffset) == 336);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxTexelGatherOffset) == 340);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, minInterpolationOffset) == 344);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxInterpolationOffset) == 348);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, subPixelInterpolationOffsetBits) == 352);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxFramebufferWidth) == 356);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxFramebufferHeight) == 360);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxFramebufferLayers) == 364);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, framebufferColorSampleCounts) == 368);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, framebufferDepthSampleCounts) == 372);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, framebufferStencilSampleCounts) == 376);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, framebufferNoAttachmentsSampleCounts) == 380);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxColorAttachments) == 384);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, sampledImageColorSampleCounts) == 388);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, sampledImageIntegerSampleCounts) == 392);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, sampledImageDepthSampleCounts) == 396);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, sampledImageStencilSampleCounts) == 400);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, storageImageSampleCounts) == 404);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxSampleMaskWords) == 408);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, timestampComputeAndGraphics) == 412);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, timestampPeriod) == 416);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxClipDistances) == 420);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxCullDistances) == 424);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, maxCombinedClipAndCullDistances) == 428);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, discreteQueuePriorities) == 432);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, pointSizeRange) == 436);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, lineWidthRange) == 444);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, pointSizeGranularity) == 452);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, lineWidthGranularity) == 456);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, strictLines) == 460);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, standardSampleLocations) == 464);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, optimalBufferCopyOffsetAlignment) == 472);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, optimalBufferCopyRowPitchAlignment) == 480);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceLimits, nonCoherentAtomSize) == 488);
const _: () = assert!(sizeof(wv64_VkPhysicalDeviceLimits) == 504);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxImageDimension1D) == 0);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxImageDimension2D) == 4);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxImageDimension3D) == 8);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxImageDimensionCube) == 12);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxImageArrayLayers) == 16);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTexelBufferElements) == 20);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxUniformBufferRange) == 24);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxStorageBufferRange) == 28);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxPushConstantsSize) == 32);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxMemoryAllocationCount) == 36);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxSamplerAllocationCount) == 40);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, bufferImageGranularity) == 48);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, sparseAddressSpaceSize) == 56);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxBoundDescriptorSets) == 64);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxPerStageDescriptorSamplers) == 68);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxPerStageDescriptorUniformBuffers) == 72);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxPerStageDescriptorStorageBuffers) == 76);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxPerStageDescriptorSampledImages) == 80);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxPerStageDescriptorStorageImages) == 84);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxPerStageDescriptorInputAttachments) == 88);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxPerStageResources) == 92);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxDescriptorSetSamplers) == 96);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxDescriptorSetUniformBuffers) == 100);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxDescriptorSetUniformBuffersDynamic) == 104);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxDescriptorSetStorageBuffers) == 108);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxDescriptorSetStorageBuffersDynamic) == 112);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxDescriptorSetSampledImages) == 116);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxDescriptorSetStorageImages) == 120);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxDescriptorSetInputAttachments) == 124);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxVertexInputAttributes) == 128);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxVertexInputBindings) == 132);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxVertexInputAttributeOffset) == 136);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxVertexInputBindingStride) == 140);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxVertexOutputComponents) == 144);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTessellationGenerationLevel) == 148);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTessellationPatchSize) == 152);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTessellationControlPerVertexInputComponents) == 156);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTessellationControlPerVertexOutputComponents) == 160);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTessellationControlPerPatchOutputComponents) == 164);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTessellationControlTotalOutputComponents) == 168);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTessellationEvaluationInputComponents) == 172);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTessellationEvaluationOutputComponents) == 176);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxGeometryShaderInvocations) == 180);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxGeometryInputComponents) == 184);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxGeometryOutputComponents) == 188);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxGeometryOutputVertices) == 192);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxGeometryTotalOutputComponents) == 196);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxFragmentInputComponents) == 200);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxFragmentOutputAttachments) == 204);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxFragmentDualSrcAttachments) == 208);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxFragmentCombinedOutputResources) == 212);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxComputeSharedMemorySize) == 216);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxComputeWorkGroupCount) == 220);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxComputeWorkGroupInvocations) == 232);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxComputeWorkGroupSize) == 236);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, subPixelPrecisionBits) == 248);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, subTexelPrecisionBits) == 252);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, mipmapPrecisionBits) == 256);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxDrawIndexedIndexValue) == 260);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxDrawIndirectCount) == 264);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxSamplerLodBias) == 268);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxSamplerAnisotropy) == 272);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxViewports) == 276);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxViewportDimensions) == 280);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, viewportBoundsRange) == 288);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, viewportSubPixelBits) == 296);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, minMemoryMapAlignment) == 304);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, minTexelBufferOffsetAlignment) == 312);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, minUniformBufferOffsetAlignment) == 320);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, minStorageBufferOffsetAlignment) == 328);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, minTexelOffset) == 336);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTexelOffset) == 340);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, minTexelGatherOffset) == 344);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxTexelGatherOffset) == 348);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, minInterpolationOffset) == 352);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxInterpolationOffset) == 356);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, subPixelInterpolationOffsetBits) == 360);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxFramebufferWidth) == 364);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxFramebufferHeight) == 368);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxFramebufferLayers) == 372);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, framebufferColorSampleCounts) == 376);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, framebufferDepthSampleCounts) == 380);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, framebufferStencilSampleCounts) == 384);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, framebufferNoAttachmentsSampleCounts) == 388);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxColorAttachments) == 392);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, sampledImageColorSampleCounts) == 396);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, sampledImageIntegerSampleCounts) == 400);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, sampledImageDepthSampleCounts) == 404);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, sampledImageStencilSampleCounts) == 408);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, storageImageSampleCounts) == 412);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxSampleMaskWords) == 416);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, timestampComputeAndGraphics) == 420);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, timestampPeriod) == 424);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxClipDistances) == 428);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxCullDistances) == 432);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, maxCombinedClipAndCullDistances) == 436);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, discreteQueuePriorities) == 440);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, pointSizeRange) == 444);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, lineWidthRange) == 452);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, pointSizeGranularity) == 460);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, lineWidthGranularity) == 464);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, strictLines) == 468);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, standardSampleLocations) == 472);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, optimalBufferCopyOffsetAlignment) == 480);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, optimalBufferCopyRowPitchAlignment) == 488);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceLimits, nonCoherentAtomSize) == 496);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; 32],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; 16],
}
const _: () = assert!(core::mem::size_of::<VkPhysicalDeviceMemoryProperties>() == 520);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceMemoryProperties, memoryTypeCount) == 0);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceMemoryProperties, memoryTypes) == 4);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceMemoryProperties, memoryHeapCount) == 260);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceMemoryProperties, memoryHeaps) == 264);
const _: () = assert!(sizeof(wv64_VkPhysicalDeviceMemoryProperties) == 520);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceMemoryProperties, memoryTypeCount) == 0);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceMemoryProperties, memoryTypes) == 4);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceMemoryProperties, memoryHeapCount) == 260);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceMemoryProperties, memoryHeaps) == 264);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: u32,
    pub deviceName: [u8; 256],
    pub pipelineCacheUUID: [u8; 16],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}
const _: () = assert!(core::mem::size_of::<VkPhysicalDeviceProperties>() == 816);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceProperties, apiVersion) == 0);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceProperties, driverVersion) == 4);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceProperties, vendorID) == 8);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceProperties, deviceID) == 12);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceProperties, deviceType) == 16);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceProperties, deviceName) == 20);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceProperties, pipelineCacheUUID) == 276);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceProperties, limits) == 296);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceProperties, sparseProperties) == 792);
const _: () = assert!(sizeof(wv64_VkPhysicalDeviceProperties) == 824);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceProperties, apiVersion) == 0);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceProperties, driverVersion) == 4);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceProperties, vendorID) == 8);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceProperties, deviceID) == 12);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceProperties, deviceType) == 16);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceProperties, deviceName) == 20);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceProperties, pipelineCacheUUID) == 276);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceProperties, limits) == 296);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceProperties, sparseProperties) == 800);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: u32,
    pub residencyStandard2DMultisampleBlockShape: u32,
    pub residencyStandard3DBlockShape: u32,
    pub residencyAlignedMipSize: u32,
    pub residencyNonResidentStrict: u32,
}
const _: () = assert!(core::mem::size_of::<VkPhysicalDeviceSparseProperties>() == 20);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceSparseProperties, residencyStandard2DBlockShape) == 0);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceSparseProperties, residencyStandard2DMultisampleBlockShape) == 4);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceSparseProperties, residencyStandard3DBlockShape) == 8);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceSparseProperties, residencyAlignedMipSize) == 12);
const _: () = assert!(core::mem::offset_of!(VkPhysicalDeviceSparseProperties, residencyNonResidentStrict) == 16);
const _: () = assert!(sizeof(wv64_VkPhysicalDeviceSparseProperties) == 20);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceSparseProperties, residencyStandard2DBlockShape) == 0);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceSparseProperties, residencyStandard2DMultisampleBlockShape) == 4);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceSparseProperties, residencyStandard3DBlockShape) == 8);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceSparseProperties, residencyAlignedMipSize) == 12);
const _: () = assert!(offsetof(wv64_VkPhysicalDeviceSparseProperties, residencyNonResidentStrict) == 16);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blendEnable: u32,
    pub srcColorBlendFactor: u32,
    pub dstColorBlendFactor: u32,
    pub colorBlendOp: u32,
    pub srcAlphaBlendFactor: u32,
    pub dstAlphaBlendFactor: u32,
    pub alphaBlendOp: u32,
    pub colorWriteMask: u32,
}
const _: () = assert!(core::mem::size_of::<VkPipelineColorBlendAttachmentState>() == 32);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendAttachmentState, blendEnable) == 0);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendAttachmentState, srcColorBlendFactor) == 4);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendAttachmentState, dstColorBlendFactor) == 8);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendAttachmentState, colorBlendOp) == 12);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendAttachmentState, srcAlphaBlendFactor) == 16);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendAttachmentState, dstAlphaBlendFactor) == 20);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendAttachmentState, alphaBlendOp) == 24);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendAttachmentState, colorWriteMask) == 28);
const _: () = assert!(sizeof(wv64_VkPipelineColorBlendAttachmentState) == 32);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendAttachmentState, blendEnable) == 0);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendAttachmentState, srcColorBlendFactor) == 4);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendAttachmentState, dstColorBlendFactor) == 8);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendAttachmentState, colorBlendOp) == 12);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendAttachmentState, srcAlphaBlendFactor) == 16);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendAttachmentState, dstAlphaBlendFactor) == 20);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendAttachmentState, alphaBlendOp) == 24);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendAttachmentState, colorWriteMask) == 28);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub logicOpEnable: u32,
    pub logicOp: u32,
    pub attachmentCount: u32,
    pub pAttachments: u32,
    pub blendConstants: [f32; 4],
}
const _: () = assert!(core::mem::size_of::<VkPipelineColorBlendStateCreateInfo>() == 44);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendStateCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendStateCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendStateCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendStateCreateInfo, logicOpEnable) == 12);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendStateCreateInfo, logicOp) == 16);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendStateCreateInfo, attachmentCount) == 20);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendStateCreateInfo, pAttachments) == 24);
const _: () = assert!(core::mem::offset_of!(VkPipelineColorBlendStateCreateInfo, blendConstants) == 28);
const _: () = assert!(sizeof(wv64_VkPipelineColorBlendStateCreateInfo) == 56);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendStateCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendStateCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendStateCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendStateCreateInfo, logicOpEnable) == 20);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendStateCreateInfo, logicOp) == 24);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendStateCreateInfo, attachmentCount) == 28);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendStateCreateInfo, pAttachments) == 32);
const _: () = assert!(offsetof(wv64_VkPipelineColorBlendStateCreateInfo, blendConstants) == 40);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub topology: u32,
    pub primitiveRestartEnable: u32,
}
const _: () = assert!(core::mem::size_of::<VkPipelineInputAssemblyStateCreateInfo>() == 20);
const _: () = assert!(core::mem::offset_of!(VkPipelineInputAssemblyStateCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkPipelineInputAssemblyStateCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkPipelineInputAssemblyStateCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkPipelineInputAssemblyStateCreateInfo, topology) == 12);
const _: () = assert!(core::mem::offset_of!(VkPipelineInputAssemblyStateCreateInfo, primitiveRestartEnable) == 16);
const _: () = assert!(sizeof(wv64_VkPipelineInputAssemblyStateCreateInfo) == 32);
const _: () = assert!(offsetof(wv64_VkPipelineInputAssemblyStateCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkPipelineInputAssemblyStateCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkPipelineInputAssemblyStateCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkPipelineInputAssemblyStateCreateInfo, topology) == 20);
const _: () = assert!(offsetof(wv64_VkPipelineInputAssemblyStateCreateInfo, primitiveRestartEnable) == 24);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineLayoutCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub setLayoutCount: u32,
    pub pSetLayouts: u32,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: u32,
}
const _: () = assert!(core::mem::size_of::<VkPipelineLayoutCreateInfo>() == 28);
const _: () = assert!(core::mem::offset_of!(VkPipelineLayoutCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkPipelineLayoutCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkPipelineLayoutCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkPipelineLayoutCreateInfo, setLayoutCount) == 12);
const _: () = assert!(core::mem::offset_of!(VkPipelineLayoutCreateInfo, pSetLayouts) == 16);
const _: () = assert!(core::mem::offset_of!(VkPipelineLayoutCreateInfo, pushConstantRangeCount) == 20);
const _: () = assert!(core::mem::offset_of!(VkPipelineLayoutCreateInfo, pPushConstantRanges) == 24);
const _: () = assert!(sizeof(wv64_VkPipelineLayoutCreateInfo) == 48);
const _: () = assert!(offsetof(wv64_VkPipelineLayoutCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkPipelineLayoutCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkPipelineLayoutCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkPipelineLayoutCreateInfo, setLayoutCount) == 20);
const _: () = assert!(offsetof(wv64_VkPipelineLayoutCreateInfo, pSetLayouts) == 24);
const _: () = assert!(offsetof(wv64_VkPipelineLayoutCreateInfo, pushConstantRangeCount) == 32);
const _: () = assert!(offsetof(wv64_VkPipelineLayoutCreateInfo, pPushConstantRanges) == 40);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub rasterizationSamples: u32,
    pub sampleShadingEnable: u32,
    pub minSampleShading: f32,
    pub pSampleMask: u32,
    pub alphaToCoverageEnable: u32,
    pub alphaToOneEnable: u32,
}
const _: () = assert!(core::mem::size_of::<VkPipelineMultisampleStateCreateInfo>() == 36);
const _: () = assert!(core::mem::offset_of!(VkPipelineMultisampleStateCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkPipelineMultisampleStateCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkPipelineMultisampleStateCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkPipelineMultisampleStateCreateInfo, rasterizationSamples) == 12);
const _: () = assert!(core::mem::offset_of!(VkPipelineMultisampleStateCreateInfo, sampleShadingEnable) == 16);
const _: () = assert!(core::mem::offset_of!(VkPipelineMultisampleStateCreateInfo, minSampleShading) == 20);
const _: () = assert!(core::mem::offset_of!(VkPipelineMultisampleStateCreateInfo, pSampleMask) == 24);
const _: () = assert!(core::mem::offset_of!(VkPipelineMultisampleStateCreateInfo, alphaToCoverageEnable) == 28);
const _: () = assert!(core::mem::offset_of!(VkPipelineMultisampleStateCreateInfo, alphaToOneEnable) == 32);
const _: () = assert!(sizeof(wv64_VkPipelineMultisampleStateCreateInfo) == 48);
const _: () = assert!(offsetof(wv64_VkPipelineMultisampleStateCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkPipelineMultisampleStateCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkPipelineMultisampleStateCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkPipelineMultisampleStateCreateInfo, rasterizationSamples) == 20);
const _: () = assert!(offsetof(wv64_VkPipelineMultisampleStateCreateInfo, sampleShadingEnable) == 24);
const _: () = assert!(offsetof(wv64_VkPipelineMultisampleStateCreateInfo, minSampleShading) == 28);
const _: () = assert!(offsetof(wv64_VkPipelineMultisampleStateCreateInfo, pSampleMask) == 32);
const _: () = assert!(offsetof(wv64_VkPipelineMultisampleStateCreateInfo, alphaToCoverageEnable) == 40);
const _: () = assert!(offsetof(wv64_VkPipelineMultisampleStateCreateInfo, alphaToOneEnable) == 44);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub depthClampEnable: u32,
    pub rasterizerDiscardEnable: u32,
    pub polygonMode: u32,
    pub cullMode: u32,
    pub frontFace: u32,
    pub depthBiasEnable: u32,
    pub depthBiasConstantFactor: f32,
    pub depthBiasClamp: f32,
    pub depthBiasSlopeFactor: f32,
    pub lineWidth: f32,
}
const _: () = assert!(core::mem::size_of::<VkPipelineRasterizationStateCreateInfo>() == 52);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, depthClampEnable) == 12);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, rasterizerDiscardEnable) == 16);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, polygonMode) == 20);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, cullMode) == 24);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, frontFace) == 28);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, depthBiasEnable) == 32);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, depthBiasConstantFactor) == 36);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, depthBiasClamp) == 40);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, depthBiasSlopeFactor) == 44);
const _: () = assert!(core::mem::offset_of!(VkPipelineRasterizationStateCreateInfo, lineWidth) == 48);
const _: () = assert!(sizeof(wv64_VkPipelineRasterizationStateCreateInfo) == 64);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, depthClampEnable) == 20);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, rasterizerDiscardEnable) == 24);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, polygonMode) == 28);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, cullMode) == 32);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, frontFace) == 36);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, depthBiasEnable) == 40);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, depthBiasConstantFactor) == 44);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, depthBiasClamp) == 48);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, depthBiasSlopeFactor) == 52);
const _: () = assert!(offsetof(wv64_VkPipelineRasterizationStateCreateInfo, lineWidth) == 56);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub stage: u32,
    pub module: u64,
    pub pName: u32,
    pub pSpecializationInfo: u32,
}
const _: () = assert!(core::mem::size_of::<VkPipelineShaderStageCreateInfo>() == 32);
const _: () = assert!(core::mem::offset_of!(VkPipelineShaderStageCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkPipelineShaderStageCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkPipelineShaderStageCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkPipelineShaderStageCreateInfo, stage) == 12);
const _: () = assert!(core::mem::offset_of!(VkPipelineShaderStageCreateInfo, module) == 16);
const _: () = assert!(core::mem::offset_of!(VkPipelineShaderStageCreateInfo, pName) == 24);
const _: () = assert!(core::mem::offset_of!(VkPipelineShaderStageCreateInfo, pSpecializationInfo) == 28);
const _: () = assert!(sizeof(wv64_VkPipelineShaderStageCreateInfo) == 48);
const _: () = assert!(offsetof(wv64_VkPipelineShaderStageCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkPipelineShaderStageCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkPipelineShaderStageCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkPipelineShaderStageCreateInfo, stage) == 20);
const _: () = assert!(offsetof(wv64_VkPipelineShaderStageCreateInfo, module) == 24);
const _: () = assert!(offsetof(wv64_VkPipelineShaderStageCreateInfo, pName) == 32);
const _: () = assert!(offsetof(wv64_VkPipelineShaderStageCreateInfo, pSpecializationInfo) == 40);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: u32,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: u32,
}
const _: () = assert!(core::mem::size_of::<VkPipelineVertexInputStateCreateInfo>() == 28);
const _: () = assert!(core::mem::offset_of!(VkPipelineVertexInputStateCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkPipelineVertexInputStateCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkPipelineVertexInputStateCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkPipelineVertexInputStateCreateInfo, vertexBindingDescriptionCount) == 12);
const _: () = assert!(core::mem::offset_of!(VkPipelineVertexInputStateCreateInfo, pVertexBindingDescriptions) == 16);
const _: () = assert!(core::mem::offset_of!(VkPipelineVertexInputStateCreateInfo, vertexAttributeDescriptionCount) == 20);
const _: () = assert!(core::mem::offset_of!(VkPipelineVertexInputStateCreateInfo, pVertexAttributeDescriptions) == 24);
const _: () = assert!(sizeof(wv64_VkPipelineVertexInputStateCreateInfo) == 48);
const _: () = assert!(offsetof(wv64_VkPipelineVertexInputStateCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkPipelineVertexInputStateCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkPipelineVertexInputStateCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkPipelineVertexInputStateCreateInfo, vertexBindingDescriptionCount) == 20);
const _: () = assert!(offsetof(wv64_VkPipelineVertexInputStateCreateInfo, pVertexBindingDescriptions) == 24);
const _: () = assert!(offsetof(wv64_VkPipelineVertexInputStateCreateInfo, vertexAttributeDescriptionCount) == 32);
const _: () = assert!(offsetof(wv64_VkPipelineVertexInputStateCreateInfo, pVertexAttributeDescriptions) == 40);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineViewportStateCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub viewportCount: u32,
    pub pViewports: u32,
    pub scissorCount: u32,
    pub pScissors: u32,
}
const _: () = assert!(core::mem::size_of::<VkPipelineViewportStateCreateInfo>() == 28);
const _: () = assert!(core::mem::offset_of!(VkPipelineViewportStateCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkPipelineViewportStateCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkPipelineViewportStateCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkPipelineViewportStateCreateInfo, viewportCount) == 12);
const _: () = assert!(core::mem::offset_of!(VkPipelineViewportStateCreateInfo, pViewports) == 16);
const _: () = assert!(core::mem::offset_of!(VkPipelineViewportStateCreateInfo, scissorCount) == 20);
const _: () = assert!(core::mem::offset_of!(VkPipelineViewportStateCreateInfo, pScissors) == 24);
const _: () = assert!(sizeof(wv64_VkPipelineViewportStateCreateInfo) == 48);
const _: () = assert!(offsetof(wv64_VkPipelineViewportStateCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkPipelineViewportStateCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkPipelineViewportStateCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkPipelineViewportStateCreateInfo, viewportCount) == 20);
const _: () = assert!(offsetof(wv64_VkPipelineViewportStateCreateInfo, pViewports) == 24);
const _: () = assert!(offsetof(wv64_VkPipelineViewportStateCreateInfo, scissorCount) == 32);
const _: () = assert!(offsetof(wv64_VkPipelineViewportStateCreateInfo, pScissors) == 40);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: u32,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}
const _: () = assert!(core::mem::size_of::<VkQueueFamilyProperties>() == 24);
const _: () = assert!(core::mem::offset_of!(VkQueueFamilyProperties, queueFlags) == 0);
const _: () = assert!(core::mem::offset_of!(VkQueueFamilyProperties, queueCount) == 4);
const _: () = assert!(core::mem::offset_of!(VkQueueFamilyProperties, timestampValidBits) == 8);
const _: () = assert!(core::mem::offset_of!(VkQueueFamilyProperties, minImageTransferGranularity) == 12);
const _: () = assert!(sizeof(wv64_VkQueueFamilyProperties) == 24);
const _: () = assert!(offsetof(wv64_VkQueueFamilyProperties, queueFlags) == 0);
const _: () = assert!(offsetof(wv64_VkQueueFamilyProperties, queueCount) == 4);
const _: () = assert!(offsetof(wv64_VkQueueFamilyProperties, timestampValidBits) == 8);
const _: () = assert!(offsetof(wv64_VkQueueFamilyProperties, minImageTransferGranularity) == 12);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}
const _: () = assert!(core::mem::size_of::<VkRect2D>() == 16);
const _: () = assert!(core::mem::offset_of!(VkRect2D, offset) == 0);
const _: () = assert!(core::mem::offset_of!(VkRect2D, extent) == 8);
const _: () = assert!(sizeof(wv64_VkRect2D) == 16);
const _: () = assert!(offsetof(wv64_VkRect2D, offset) == 0);
const _: () = assert!(offsetof(wv64_VkRect2D, extent) == 8);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRenderPassBeginInfo {
    pub sType: u32,
    pub pNext: u32,
    pub renderPass: u64,
    pub framebuffer: u64,
    pub renderArea: VkRect2D,
    pub clearValueCount: u32,
    pub pClearValues: u32,
}
const _: () = assert!(core::mem::size_of::<VkRenderPassBeginInfo>() == 48);
const _: () = assert!(core::mem::offset_of!(VkRenderPassBeginInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkRenderPassBeginInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkRenderPassBeginInfo, renderPass) == 8);
const _: () = assert!(core::mem::offset_of!(VkRenderPassBeginInfo, framebuffer) == 16);
const _: () = assert!(core::mem::offset_of!(VkRenderPassBeginInfo, renderArea) == 24);
const _: () = assert!(core::mem::offset_of!(VkRenderPassBeginInfo, clearValueCount) == 40);
const _: () = assert!(core::mem::offset_of!(VkRenderPassBeginInfo, pClearValues) == 44);
const _: () = assert!(sizeof(wv64_VkRenderPassBeginInfo) == 64);
const _: () = assert!(offsetof(wv64_VkRenderPassBeginInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkRenderPassBeginInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkRenderPassBeginInfo, renderPass) == 16);
const _: () = assert!(offsetof(wv64_VkRenderPassBeginInfo, framebuffer) == 24);
const _: () = assert!(offsetof(wv64_VkRenderPassBeginInfo, renderArea) == 32);
const _: () = assert!(offsetof(wv64_VkRenderPassBeginInfo, clearValueCount) == 48);
const _: () = assert!(offsetof(wv64_VkRenderPassBeginInfo, pClearValues) == 56);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRenderPassCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub attachmentCount: u32,
    pub pAttachments: u32,
    pub subpassCount: u32,
    pub pSubpasses: u32,
    pub dependencyCount: u32,
    pub pDependencies: u32,
}
const _: () = assert!(core::mem::size_of::<VkRenderPassCreateInfo>() == 36);
const _: () = assert!(core::mem::offset_of!(VkRenderPassCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkRenderPassCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkRenderPassCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkRenderPassCreateInfo, attachmentCount) == 12);
const _: () = assert!(core::mem::offset_of!(VkRenderPassCreateInfo, pAttachments) == 16);
const _: () = assert!(core::mem::offset_of!(VkRenderPassCreateInfo, subpassCount) == 20);
const _: () = assert!(core::mem::offset_of!(VkRenderPassCreateInfo, pSubpasses) == 24);
const _: () = assert!(core::mem::offset_of!(VkRenderPassCreateInfo, dependencyCount) == 28);
const _: () = assert!(core::mem::offset_of!(VkRenderPassCreateInfo, pDependencies) == 32);
const _: () = assert!(sizeof(wv64_VkRenderPassCreateInfo) == 64);
const _: () = assert!(offsetof(wv64_VkRenderPassCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkRenderPassCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkRenderPassCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkRenderPassCreateInfo, attachmentCount) == 20);
const _: () = assert!(offsetof(wv64_VkRenderPassCreateInfo, pAttachments) == 24);
const _: () = assert!(offsetof(wv64_VkRenderPassCreateInfo, subpassCount) == 32);
const _: () = assert!(offsetof(wv64_VkRenderPassCreateInfo, pSubpasses) == 40);
const _: () = assert!(offsetof(wv64_VkRenderPassCreateInfo, dependencyCount) == 48);
const _: () = assert!(offsetof(wv64_VkRenderPassCreateInfo, pDependencies) == 56);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkShaderModuleCreateInfo {
    pub sType: u32,
    pub pNext: u32,
    pub flags: u32,
    pub codeSize: u32,
    pub pCode: u32,
}
const _: () = assert!(core::mem::size_of::<VkShaderModuleCreateInfo>() == 20);
const _: () = assert!(core::mem::offset_of!(VkShaderModuleCreateInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkShaderModuleCreateInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkShaderModuleCreateInfo, flags) == 8);
const _: () = assert!(core::mem::offset_of!(VkShaderModuleCreateInfo, codeSize) == 12);
const _: () = assert!(core::mem::offset_of!(VkShaderModuleCreateInfo, pCode) == 16);
const _: () = assert!(sizeof(wv64_VkShaderModuleCreateInfo) == 40);
const _: () = assert!(offsetof(wv64_VkShaderModuleCreateInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkShaderModuleCreateInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkShaderModuleCreateInfo, flags) == 16);
const _: () = assert!(offsetof(wv64_VkShaderModuleCreateInfo, codeSize) == 24);
const _: () = assert!(offsetof(wv64_VkShaderModuleCreateInfo, pCode) == 32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubmitInfo {
    pub sType: u32,
    pub pNext: u32,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: u32,
    pub pWaitDstStageMask: u32,
    pub commandBufferCount: u32,
    pub pCommandBuffers: u32,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: u32,
}
const _: () = assert!(core::mem::size_of::<VkSubmitInfo>() == 36);
const _: () = assert!(core::mem::offset_of!(VkSubmitInfo, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkSubmitInfo, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkSubmitInfo, waitSemaphoreCount) == 8);
const _: () = assert!(core::mem::offset_of!(VkSubmitInfo, pWaitSemaphores) == 12);
const _: () = assert!(core::mem::offset_of!(VkSubmitInfo, pWaitDstStageMask) == 16);
const _: () = assert!(core::mem::offset_of!(VkSubmitInfo, commandBufferCount) == 20);
const _: () = assert!(core::mem::offset_of!(VkSubmitInfo, pCommandBuffers) == 24);
const _: () = assert!(core::mem::offset_of!(VkSubmitInfo, signalSemaphoreCount) == 28);
const _: () = assert!(core::mem::offset_of!(VkSubmitInfo, pSignalSemaphores) == 32);
const _: () = assert!(sizeof(wv64_VkSubmitInfo) == 72);
const _: () = assert!(offsetof(wv64_VkSubmitInfo, sType) == 0);
const _: () = assert!(offsetof(wv64_VkSubmitInfo, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkSubmitInfo, waitSemaphoreCount) == 16);
const _: () = assert!(offsetof(wv64_VkSubmitInfo, pWaitSemaphores) == 24);
const _: () = assert!(offsetof(wv64_VkSubmitInfo, pWaitDstStageMask) == 32);
const _: () = assert!(offsetof(wv64_VkSubmitInfo, commandBufferCount) == 40);
const _: () = assert!(offsetof(wv64_VkSubmitInfo, pCommandBuffers) == 48);
const _: () = assert!(offsetof(wv64_VkSubmitInfo, signalSemaphoreCount) == 56);
const _: () = assert!(offsetof(wv64_VkSubmitInfo, pSignalSemaphores) == 64);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: u32,
    pub dstStageMask: u32,
    pub srcAccessMask: u32,
    pub dstAccessMask: u32,
    pub dependencyFlags: u32,
}
const _: () = assert!(core::mem::size_of::<VkSubpassDependency>() == 28);
const _: () = assert!(core::mem::offset_of!(VkSubpassDependency, srcSubpass) == 0);
const _: () = assert!(core::mem::offset_of!(VkSubpassDependency, dstSubpass) == 4);
const _: () = assert!(core::mem::offset_of!(VkSubpassDependency, srcStageMask) == 8);
const _: () = assert!(core::mem::offset_of!(VkSubpassDependency, dstStageMask) == 12);
const _: () = assert!(core::mem::offset_of!(VkSubpassDependency, srcAccessMask) == 16);
const _: () = assert!(core::mem::offset_of!(VkSubpassDependency, dstAccessMask) == 20);
const _: () = assert!(core::mem::offset_of!(VkSubpassDependency, dependencyFlags) == 24);
const _: () = assert!(sizeof(wv64_VkSubpassDependency) == 28);
const _: () = assert!(offsetof(wv64_VkSubpassDependency, srcSubpass) == 0);
const _: () = assert!(offsetof(wv64_VkSubpassDependency, dstSubpass) == 4);
const _: () = assert!(offsetof(wv64_VkSubpassDependency, srcStageMask) == 8);
const _: () = assert!(offsetof(wv64_VkSubpassDependency, dstStageMask) == 12);
const _: () = assert!(offsetof(wv64_VkSubpassDependency, srcAccessMask) == 16);
const _: () = assert!(offsetof(wv64_VkSubpassDependency, dstAccessMask) == 20);
const _: () = assert!(offsetof(wv64_VkSubpassDependency, dependencyFlags) == 24);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubpassDescription {
    pub flags: u32,
    pub pipelineBindPoint: u32,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: u32,
    pub pResolveAttachments: u32,
    pub pDepthStencilAttachment: u32,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: u32,
}
const _: () = assert!(core::mem::size_of::<VkSubpassDescription>() == 40);
const _: () = assert!(core::mem::offset_of!(VkSubpassDescription, flags) == 0);
const _: () = assert!(core::mem::offset_of!(VkSubpassDescription, pipelineBindPoint) == 4);
const _: () = assert!(core::mem::offset_of!(VkSubpassDescription, inputAttachmentCount) == 8);
const _: () = assert!(core::mem::offset_of!(VkSubpassDescription, pInputAttachments) == 12);
const _: () = assert!(core::mem::offset_of!(VkSubpassDescription, colorAttachmentCount) == 16);
const _: () = assert!(core::mem::offset_of!(VkSubpassDescription, pColorAttachments) == 20);
const _: () = assert!(core::mem::offset_of!(VkSubpassDescription, pResolveAttachments) == 24);
const _: () = assert!(core::mem::offset_of!(VkSubpassDescription, pDepthStencilAttachment) == 28);
const _: () = assert!(core::mem::offset_of!(VkSubpassDescription, preserveAttachmentCount) == 32);
const _: () = assert!(core::mem::offset_of!(VkSubpassDescription, pPreserveAttachments) == 36);
const _: () = assert!(sizeof(wv64_VkSubpassDescription) == 72);
const _: () = assert!(offsetof(wv64_VkSubpassDescription, flags) == 0);
const _: () = assert!(offsetof(wv64_VkSubpassDescription, pipelineBindPoint) == 4);
const _: () = assert!(offsetof(wv64_VkSubpassDescription, inputAttachmentCount) == 8);
const _: () = assert!(offsetof(wv64_VkSubpassDescription, pInputAttachments) == 16);
const _: () = assert!(offsetof(wv64_VkSubpassDescription, colorAttachmentCount) == 24);
const _: () = assert!(offsetof(wv64_VkSubpassDescription, pColorAttachments) == 32);
const _: () = assert!(offsetof(wv64_VkSubpassDescription, pResolveAttachments) == 40);
const _: () = assert!(offsetof(wv64_VkSubpassDescription, pDepthStencilAttachment) == 48);
const _: () = assert!(offsetof(wv64_VkSubpassDescription, preserveAttachmentCount) == 56);
const _: () = assert!(offsetof(wv64_VkSubpassDescription, pPreserveAttachments) == 64);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub minDepth: f32,
    pub maxDepth: f32,
}
const _: () = assert!(core::mem::size_of::<VkViewport>() == 24);
const _: () = assert!(core::mem::offset_of!(VkViewport, x) == 0);
const _: () = assert!(core::mem::offset_of!(VkViewport, y) == 4);
const _: () = assert!(core::mem::offset_of!(VkViewport, width) == 8);
const _: () = assert!(core::mem::offset_of!(VkViewport, height) == 12);
const _: () = assert!(core::mem::offset_of!(VkViewport, minDepth) == 16);
const _: () = assert!(core::mem::offset_of!(VkViewport, maxDepth) == 20);
const _: () = assert!(sizeof(wv64_VkViewport) == 24);
const _: () = assert!(offsetof(wv64_VkViewport, x) == 0);
const _: () = assert!(offsetof(wv64_VkViewport, y) == 4);
const _: () = assert!(offsetof(wv64_VkViewport, width) == 8);
const _: () = assert!(offsetof(wv64_VkViewport, height) == 12);
const _: () = assert!(offsetof(wv64_VkViewport, minDepth) == 16);
const _: () = assert!(offsetof(wv64_VkViewport, maxDepth) == 20);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkWriteDescriptorSet {
    pub sType: u32,
    pub pNext: u32,
    pub dstSet: u64,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: u32,
    pub pImageInfo: u32,
    pub pBufferInfo: u32,
    pub pTexelBufferView: u32,
}
const _: () = assert!(core::mem::size_of::<VkWriteDescriptorSet>() == 48);
const _: () = assert!(core::mem::offset_of!(VkWriteDescriptorSet, sType) == 0);
const _: () = assert!(core::mem::offset_of!(VkWriteDescriptorSet, pNext) == 4);
const _: () = assert!(core::mem::offset_of!(VkWriteDescriptorSet, dstSet) == 8);
const _: () = assert!(core::mem::offset_of!(VkWriteDescriptorSet, dstBinding) == 16);
const _: () = assert!(core::mem::offset_of!(VkWriteDescriptorSet, dstArrayElement) == 20);
const _: () = assert!(core::mem::offset_of!(VkWriteDescriptorSet, descriptorCount) == 24);
const _: () = assert!(core::mem::offset_of!(VkWriteDescriptorSet, descriptorType) == 28);
const _: () = assert!(core::mem::offset_of!(VkWriteDescriptorSet, pImageInfo) == 32);
const _: () = assert!(core::mem::offset_of!(VkWriteDescriptorSet, pBufferInfo) == 36);
const _: () = assert!(core::mem::offset_of!(VkWriteDescriptorSet, pTexelBufferView) == 40);
const _: () = assert!(sizeof(wv64_VkWriteDescriptorSet) == 64);
const _: () = assert!(offsetof(wv64_VkWriteDescriptorSet, sType) == 0);
const _: () = assert!(offsetof(wv64_VkWriteDescriptorSet, pNext) == 8);
const _: () = assert!(offsetof(wv64_VkWriteDescriptorSet, dstSet) == 16);
const _: () = assert!(offsetof(wv64_VkWriteDescriptorSet, dstBinding) == 24);
const _: () = assert!(offsetof(wv64_VkWriteDescriptorSet, dstArrayElement) == 28);
const _: () = assert!(offsetof(wv64_VkWriteDescriptorSet, descriptorCount) == 32);
const _: () = assert!(offsetof(wv64_VkWriteDescriptorSet, descriptorType) == 36);
const _: () = assert!(offsetof(wv64_VkWriteDescriptorSet, pImageInfo) == 40);
const _: () = assert!(offsetof(wv64_VkWriteDescriptorSet, pBufferInfo) == 48);
const _: () = assert!(offsetof(wv64_VkWriteDescriptorSet, pTexelBufferView) == 56);
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: u32 = 0x00000100;
pub const VK_ACCESS_HOST_READ_BIT: u32 = 0x00002000;
pub const VK_ACCESS_TRANSFER_READ_BIT: u32 = 0x00000800;
pub const VK_ACCESS_TRANSFER_WRITE_BIT: u32 = 0x00001000;
pub const VK_API_VERSION_1_0: u32 = 4194304;
pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: u32 = 2;
pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: u32 = 1;
pub const VK_ATTACHMENT_STORE_OP_STORE: u32 = 0;
pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: u32 = 0x00000020;
pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: u32 = 0x00000002;
pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: u32 = 0x00000001;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: u32 = 7;
pub const VK_FORMAT_R8G8B8A8_UNORM: u32 = 37;
pub const VK_IMAGE_ASPECT_COLOR_BIT: u32 = 0x00000001;
pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: u32 = 2;
pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: u32 = 6;
pub const VK_IMAGE_LAYOUT_UNDEFINED: u32 = 0;
pub const VK_IMAGE_TILING_OPTIMAL: u32 = 0;
pub const VK_IMAGE_TYPE_2D: u32 = 1;
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: u32 = 0x00000010;
pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: u32 = 0x00000001;
pub const VK_IMAGE_VIEW_TYPE_2D: u32 = 1;
pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: u32 = 0x00000001;
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: u32 = 0x00000004;
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: u32 = 0x00000002;
pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: u32 = 2;
pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: u32 = 1;
pub const VK_PIPELINE_BIND_POINT_GRAPHICS: u32 = 0;
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: u32 = 0x00000400;
pub const VK_PIPELINE_STAGE_HOST_BIT: u32 = 0x00004000;
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: u32 = 0x00001000;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: u32 = 3;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = 0xffffffff;
pub const VK_QUEUE_GRAPHICS_BIT: u32 = 0x00000001;
pub const VK_SAMPLE_COUNT_1_BIT: u32 = 0x00000001;
pub const VK_SHADER_STAGE_FRAGMENT_BIT: u32 = 0x00000010;
pub const VK_SHADER_STAGE_VERTEX_BIT: u32 = 0x00000001;
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: u32 = 0;
pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: u32 = 12;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: u32 = 44;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: u32 = 40;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: u32 = 42;
pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: u32 = 39;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: u32 = 33;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: u32 = 34;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: u32 = 32;
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: u32 = 3;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: u32 = 2;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: u32 = 37;
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: u32 = 28;
pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: u32 = 14;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: u32 = 15;
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: u32 = 1;
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: u32 = 5;
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: u32 = 26;
pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: u32 = 20;
pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: u32 = 30;
pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: u32 = 24;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: u32 = 23;
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: u32 = 18;
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: u32 = 19;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: u32 = 22;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: u32 = 43;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: u32 = 38;
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: u32 = 16;
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: u32 = 4;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: u32 = 35;
pub const VK_SUBPASS_CONTENTS_INLINE: u32 = 0;
pub const VK_SUBPASS_EXTERNAL: u32 = 0xffffffff;
pub const VK_SUCCESS: i32 = 0;
