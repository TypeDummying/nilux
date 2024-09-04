#include <windows.h>
#include <stdio.h>
#include <string.h>
#include <urlmon.h>
#pragma comment(lib, "urlmon.lib")

void clearScreen() {
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    COORD coordScreen = {0, 0};
    DWORD cCharsWritten;
    CONSOLE_SCREEN_BUFFER_INFO csbi;
    DWORD dwConSize;

    GetConsoleScreenBufferInfo(hConsole, &csbi);
    dwConSize = csbi.dwSize.X * csbi.dwSize.Y;

    FillConsoleOutputCharacter(hConsole, (TCHAR)' ', dwConSize, coordScreen, &cCharsWritten);
    GetConsoleScreenBufferInfo(hConsole, &csbi);
    FillConsoleOutputAttribute(hConsole, csbi.wAttributes, dwConSize, coordScreen, &cCharsWritten);
    SetConsoleCursorPosition(hConsole, coordScreen);
}

void setConsoleTitle(const char* title) {
    SetConsoleTitle(title);
}

void setColor(WORD color) {
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(hConsole, color);
}

void resetColor() {
    setColor(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE); // Reset to default color
}

void printColored(const char* text, WORD color) {
    setColor(color);
    printf("%s", text);
    resetColor();
}

void downloadWebPage(const char* url) {
    HRESULT hr = URLDownloadToFile(NULL, url, "downloaded_page.html", 0, NULL);
    if (SUCCEEDED(hr)) {
        printf("Downloaded %s to downloaded_page.html\n", url);
    } else {
        fprintf(stderr, "Failed to download %s\n", url);
    }
}

void showLoadingSpinner(int seconds) {
    const char spinner[] = "|/-\\";
    int spinnerIndex = 0;
    DWORD startTime = GetTickCount();
    DWORD endTime = startTime + (seconds * 1000);

    while (GetTickCount() < endTime) {
        printf("\rLoading... %c", spinner[spinnerIndex]);
        fflush(stdout);
        spinnerIndex = (spinnerIndex + 1) % 4;
        Sleep(100);
    }
    printf("\rLoading... Done!\n");
}

void compileCode(const char* filename) {
    char command[512];
    snprintf(command, sizeof(command), "gcc -o %s.exe %s", filename, filename);
    int result = system(command);
    if (result == 0) {
        printf("Compilation successful: %s.exe\n", filename);
    } else {
        fprintf(stderr, "Compilation failed for %s\n", filename);
    }
}

void managePackages(const char* action, const char* package) {
    if (strcmp(action, "install") == 0) {
        printf("Installing package: %s\n", package);
        // Simulate package installation
        Sleep(2000);
        printf("Package %s installed successfully.\n", package);
    } else if (strcmp(action, "remove") == 0) {
        printf("Removing package: %s\n", package);
        // Simulate package removal
        Sleep(2000);
        printf("Package %s removed successfully.\n", package);
    } else {
        fprintf(stderr, "Unknown package action: %s\n", action);
    }
}

int main() {
    char input[256];

    // Set the console title
    setConsoleTitle("nilux_dev_amd64");

    // Show loading spinner for 3 seconds
    showLoadingSpinner(3);

    printf("hello, nilux devs!\n");
    printf("Type 'exit' to quit.\n");

    while (1) {
        printf("nilux@devmode > ");
        fgets(input, sizeof(input), stdin);

        // Remove newline character from input
        input[strcspn(input, "\n")] = 0;

        // Check for exit command
        if (strcmp(input, "exit") == 0) {
            printColored("exit", FOREGROUND_RED | FOREGROUND_GREEN); // Yellowish orange
            printf("\nExiting...\n");
            break;
        }

        // Check for echo command
        if (strncmp(input, "echo", 4) == 0) {
            printColored("echo", FOREGROUND_BLUE); // Blue
            printf(" %s\n", input + 5);
        } else if (strcmp(input, "clear") == 0) {
            clearScreen();
        } else if (strncmp(input, "adb-get ", 8) == 0) {
            char* url = input + 8;
            downloadWebPage(url);
        } else if (strncmp(input, "compile ", 8) == 0) {
            char* filename = input + 8;
            compileCode(filename);
        } else if (strncmp(input, "pkg ", 4) == 0) {
            char* action = strtok(input + 4, " ");
            char* package = strtok(NULL, " ");
            if (action && package) {
                managePackages(action, package);
            } else {
                fprintf(stderr, "Invalid package command.\n");
            }
        } else {
            printf("You typed: %s\n", input);
        }
    }

    return 0;
}
