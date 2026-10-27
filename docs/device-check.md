# Initial device check, 2026-10-27

The first command was `vulkaninfo --summary`. It failed with exit status 127:
`vulkaninfo: command not found`.

A direct query through Python ctypes and `libvulkan.so.1` then returned:

```text
vkCreateInstance: VK_SUCCESS (0)
vkEnumeratePhysicalDevices: VK_SUCCESS (0), count 1
device: llvmpipe (LLVM 19.1.7, 256 bits)
device type: VK_PHYSICAL_DEVICE_TYPE_CPU (4)
/dev/dri/* and /dev/nvidia*: no device nodes
```

The container can run headless Vulkan through Mesa's software device. It cannot
reach the laptop's NVIDIA RTX 4060. A container run will therefore be identified
as a software Vulkan run. Hardware validation must run on the host.

Upstream was cloned and its README and headless example read before implementation:
https://github.com/jtenner/wago-vulkan at
`a3943c4da750e5016ccc9e35fb932448e71f3e43`.
