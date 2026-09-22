#include <iostream>
#include <cstring>

extern "C" {
    int execute_system_command(const char* command) {
        if (std::strcmp(command, "help") == 0) {
            std::cout << "--- TerminalOS Help v0.2 ---\n";
            std::cout << "help    - Show this message\n";
            std::cout << "sysinfo - Display OS information\n";
            std::cout << "exit    - Shutdown TerminalOS\n";
            return 0;
        } 
        else if (std::strcmp(command, "sysinfo") == 0) {
            std::cout << "[Kernel] Running on CachyOS Kernel Sim\n";
            std::cout << "[Architecture] Hybrid Rust/C++ Architecture\n";
            return 0;
        } 
        else if (std::strcmp(command, "exit") == 0) {
            std::cout << "[Kernel] Shutting down systems... Goodbye.\n";
            return 1;
        }
        
        std::cout << "TerminalOS: command not found: " << command << "\n";
        return -1;
    }
}

