#include <stdio.h>
#include "main.h"

#include "rustui/target/rustui.h"

int main()
{
    printf("Hello, World, NETB has started!\n");
    rustui_init();
    printf("Next in NETB\n");

    send_message_to_rustui();

    while (true)
    {
    }

    return 0;
}

int function_to_be_called_from_rustui()
{
    printf("C function called from Rust!\n");
    return 42;
}