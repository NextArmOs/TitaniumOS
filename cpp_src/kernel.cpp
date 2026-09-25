#include <cstring>

extern "C" {
    int execute_system_command(const char* command, char* output_buffer, int max_len) {
        if (std::strcmp(command, "help") == 0) {
            std::strncpy(output_buffer, "Commands: help, sysinfo, clear", max_len);
            return 0;
        } 
        else if (std::strcmp(command, "sysinfo") == 0) {
            std::strncpy(output_buffer, "TitaniumOS v0.06\nKernel: Virtual C++ Kernel v0.06\nHost: Linux\nStatus: Operational", max_len);
            return 0;
        }
        else if (std::strcmp(command, "clear") == 0) {
            return 2;
        }
        
        std::strncpy(output_buffer, "TitaniumOS: command not found", max_len);
        return -1;
    }
}
