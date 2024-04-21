#include <GLFW/glfw3.h>
#include <iostream>
int main() {
    // Initialize GLFW
    if (!glfwInit()) {
        const char* description;
        glfwGetError(&description);
        std::cerr << "GLFW initialization failed: " << description << std::endl;
        return -1;
    }

    // Create a windowed mode window and its OpenGL context
    GLFWwindow* window = glfwCreateWindow(640, 480, "Hello World", NULL, NULL);
    if (!window) {
        const char* description;
        glfwGetError(&description);
        std::cerr << "Failed to create GLFW window: " << description << std::endl;
        glfwTerminate();
        return -1;
    }


    // Make the window's context current
    glfwMakeContextCurrent(window);
    
    if (!glfwGetCurrentContext()) {
        std::cerr << "Failed to create OpenGL context" << std::endl;
        glfwTerminate();
        return -1;
    }


    // Loop until the user closes the window
    while (!glfwWindowShouldClose(window)) {
        std::cout << "Inside event loop" << std::endl;
        // Set clear color (black)
        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);

        // Clear the color buffer
        glClear(GL_COLOR_BUFFER_BIT);

        // Swap front and back buffers
        glfwSwapBuffers(window);

        // Poll for and process events
        glfwPollEvents();
    }
    glfwShowWindow(window);
    // Cleanup
    glfwTerminate();
    return 0;
}
