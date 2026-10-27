#ifndef IMPORTS_H
#define IMPORTS_H
#define VK_NO_PROTOTYPES
#include <vulkan/vulkan_core.h>
#include "vendor/wire.h"
// Handles are always i64; pointers are wasm32 offsets. Never use native Vk structs.
#define IMPORT(ret, name, ...) __attribute__((import_module("vulkan.wasm32"), import_name(#name))) ret name(__VA_ARGS__)
IMPORT(int32_t, abiVersion, void);
IMPORT(int32_t, vkCreateInstance, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkEnumeratePhysicalDevices, uint64_t, uint32_t*, uint64_t*);
IMPORT(void, vkGetPhysicalDeviceProperties, uint64_t, void*);
IMPORT(void, vkGetPhysicalDeviceQueueFamilyProperties, uint64_t, uint32_t*, void*);
IMPORT(void, vkGetPhysicalDeviceMemoryProperties, uint64_t, void*);
IMPORT(int32_t, vkCreateDevice, uint64_t, const void*, const void*, uint64_t*);
IMPORT(void, vkGetDeviceQueue, uint64_t, uint32_t, uint32_t, uint64_t*);
IMPORT(int32_t, vkCreateBuffer, uint64_t, const void*, const void*, uint64_t*);
IMPORT(void, vkGetBufferMemoryRequirements, uint64_t, uint64_t, void*);
IMPORT(int32_t, vkAllocateMemory, uint64_t, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkBindBufferMemory, uint64_t, uint64_t, uint64_t, uint64_t);
IMPORT(int32_t, vkMapMemory, uint64_t, uint64_t, uint64_t, uint64_t, uint32_t, uint64_t*);
IMPORT(void, writeMapped, uint64_t, const void*, uint32_t);
IMPORT(void, readMapped, uint64_t, void*, uint32_t);
IMPORT(void, vkUnmapMemory, uint64_t, uint64_t);
IMPORT(int32_t, vkCreateImage, uint64_t, const void*, const void*, uint64_t*);
IMPORT(void, vkGetImageMemoryRequirements, uint64_t, uint64_t, void*);
IMPORT(int32_t, vkBindImageMemory, uint64_t, uint64_t, uint64_t, uint64_t);
IMPORT(int32_t, vkCreateImageView, uint64_t, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkCreateRenderPass, uint64_t, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkCreateFramebuffer, uint64_t, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkCreateShaderModule, uint64_t, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkCreateDescriptorSetLayout, uint64_t, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkCreatePipelineLayout, uint64_t, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkCreateGraphicsPipelines, uint64_t, uint64_t, uint32_t, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkCreateDescriptorPool, uint64_t, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkAllocateDescriptorSets, uint64_t, const void*, uint64_t*);
IMPORT(void, vkUpdateDescriptorSets, uint64_t, uint32_t, const void*, uint32_t, const void*);
IMPORT(int32_t, vkCreateCommandPool, uint64_t, const void*, const void*, uint64_t*);
IMPORT(int32_t, vkAllocateCommandBuffers, uint64_t, const void*, uint64_t*);
IMPORT(int32_t, vkBeginCommandBuffer, uint64_t, const void*);
IMPORT(void, vkCmdBeginRenderPass, uint64_t, const void*, uint32_t);
IMPORT(void, vkCmdBindPipeline, uint64_t, uint32_t, uint64_t);
IMPORT(void, vkCmdBindDescriptorSets, uint64_t, uint32_t, uint64_t, uint32_t, uint32_t, const void*, uint32_t, const void*);
IMPORT(void, vkCmdDraw, uint64_t, uint32_t, uint32_t, uint32_t, uint32_t);
IMPORT(void, vkCmdEndRenderPass, uint64_t);
IMPORT(void, vkCmdPipelineBarrier, uint64_t, uint32_t, uint32_t, uint32_t, uint32_t, const void*, uint32_t, const void*, uint32_t, const void*);
IMPORT(void, vkCmdCopyImageToBuffer, uint64_t, uint64_t, uint32_t, uint64_t, uint32_t, const void*);
IMPORT(int32_t, vkEndCommandBuffer, uint64_t);
IMPORT(int32_t, vkQueueSubmit, uint64_t, uint32_t, const void*, uint64_t);
IMPORT(int32_t, vkDeviceWaitIdle, uint64_t);
IMPORT(void, vkDestroyCommandPool, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyDescriptorPool, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyPipeline, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyPipelineLayout, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyDescriptorSetLayout, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyShaderModule, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyFramebuffer, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyRenderPass, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyImageView, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyImage, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyBuffer, uint64_t, uint64_t, const void*);
IMPORT(void, vkFreeMemory, uint64_t, uint64_t, const void*);
IMPORT(void, vkDestroyDevice, uint64_t, const void*);
IMPORT(void, vkDestroyInstance, uint64_t, const void*);
#endif
