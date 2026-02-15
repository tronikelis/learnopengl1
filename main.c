#include <stdbool.h>
#include <stdio.h>

#define GL_GLEXT_PROTOTYPES
#include <GL/gl.h>
#include <GLFW/glfw3.h>
#include <cglm/cglm.h>

#define STB_IMAGE_IMPLEMENTATION
#include "vendor/stb_image.h"

const char* vertexShaderSource =
    "#version 330 core\n"
    "layout (location = 0) in vec3 inPosition;\n"
    "layout (location = 1) in vec3 inColor;\n"
    "layout (location = 2) in vec2 inTextureCoordinates;\n"
    "out vec3 outColor;\n"
    "out vec2 outTextureCoordinates;\n"
    "uniform mat4 transform;\n"
    "void main()\n"
    "{\n"
    "   gl_Position = transform * vec4(inPosition, 1.0);\n"
    "   outColor = inColor;\n"
    "   outTextureCoordinates = inTextureCoordinates;\n"
    "}\n";

const char* fragmentShaderSource =
    "#version 330 core\n"
    "out vec4 fragColor;\n"
    "in vec3 outColor;\n"
    "in vec2 outTextureCoordinates;\n"
    "uniform sampler2D ourTexture;\n"
    "void main()\n"
    "{\n"
    "    fragColor = texture(ourTexture, outTextureCoordinates) * "
    "vec4(outColor, 1.0);\n"
    "}\n";

static void frameBufferSizeCallback(GLFWwindow* window, int width, int height) {
    printf("changing to %dx%d\n", width, height);
    glViewport(0, 0, width, height);
}

static void processInput(GLFWwindow* window) {
    if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, true);
    }
}

static bool compileShader(GLuint* shaderId, const char* src, GLenum type,
                          char info[512]) {
    GLuint shader = glCreateShader(type);
    *shaderId = shader;
    glShaderSource(shader, 1, &src, NULL);
    glCompileShader(shader);

    int success;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &success);

    if (!success) {
        glGetShaderInfoLog(shader, 512, NULL, info);
    }
    return success;
}

static GLFWwindow* initGlfwWindow(int width, int height, const char* title) {
    glfwInit();

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    GLFWwindow* glwindow = glfwCreateWindow(width, height, title, NULL, NULL);
    glfwMakeContextCurrent(glwindow);
    glViewport(0, 0, width, height);

    return glwindow;
}

int main() {
    GLFWwindow* glwindow = initGlfwWindow(1280, 720, "hello world");
    glfwSetFramebufferSizeCallback(glwindow, frameBufferSizeCallback);

    mat4 idenMat4 = GLM_MAT4_IDENTITY_INIT;
    // glm_scale(idenMat4, (vec3){2, 2, 2});
    // glm_translate(idenMat4, (vec3){0.1, 0.1, 0});
    glm_rotate(idenMat4, glm_rad(90), (vec3){0, 0, 1});

    int imageWidth, imageHeight, nrChannels;
    unsigned char* imageData = stbi_load("resources/container.jpg", &imageWidth,
                                         &imageHeight, &nrChannels, 0);
    if (!imageData) {
        printf("failed to load container.jpg\n");
        return 1;
    }

    GLuint texture;
    glGenTextures(1, &texture);
    glBindTexture(GL_TEXTURE_2D, texture);

    glTexImage2D(GL_TEXTURE_2D,
                 0,      // mipmap level
                 GL_RGB, // format to store in opengl
                 imageWidth, imageHeight,
                 0,                // always zero
                 GL_RGB,           // format of source image
                 GL_UNSIGNED_BYTE, // format of source image
                 imageData);
    glGenerateMipmap(GL_TEXTURE_2D);
    stbi_image_free(imageData);

    // clang-format off
    float vertices[] = {
        // positions         // colors
        0.5,  -0.5, 0.0,      1.0, 0.0, 0.0,  1.0, 0.0,  // bottom right
        -0.5, -0.5, 0.0,       0.0, 1.0, 0.0,   0.0, 0.0, // bottom left
        0.0,  0.5,  0.0,         0.0, 0.0, 1.0,   0.5, 1, // top
    };
    // clang-format on

    char info[512];
    GLuint vertexShader;
    if (!compileShader(&vertexShader, vertexShaderSource, GL_VERTEX_SHADER,
                       info)) {
        printf("%s\n", info);
        return 1;
    }

    GLuint fragmentShader;
    if (!compileShader(&fragmentShader, fragmentShaderSource,
                       GL_FRAGMENT_SHADER, info)) {
        printf("%s\n", info);
        return 1;
    }

    GLuint shaderProgram = glCreateProgram();
    glAttachShader(shaderProgram, vertexShader);
    glAttachShader(shaderProgram, fragmentShader);
    glLinkProgram(shaderProgram);

    int success;
    glGetProgramiv(shaderProgram, GL_LINK_STATUS, &success);
    if (!success) {
        glGetProgramInfoLog(shaderProgram, 512, NULL, info);
        printf("%s\n", info);
        return 1;
    }

    glUseProgram(shaderProgram);
    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);

    GLuint vao;
    glGenVertexArrays(1, &vao);
    glBindVertexArray(vao);

    GLuint vbo;
    glGenBuffers(1, &vbo);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    // position
    glVertexAttribPointer(0,                 // location
                          3,                 // count
                          GL_FLOAT,          // type
                          GL_FALSE,          // normalize to [-1, 1]
                          8 * sizeof(float), // stride (bytes to go to next)
                          (void*)0);         // offset
    glEnableVertexAttribArray(0);

    // color
    glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 8 * sizeof(float),
                          (void*)(3 * sizeof(float)));
    glEnableVertexAttribArray(1);

    // texture coordinates
    glVertexAttribPointer(2, 2, GL_FLOAT, GL_FALSE, 8 * sizeof(float),
                          (void*)(6 * sizeof(float)));
    glEnableVertexAttribArray(2);

    // unbinding
    glBindVertexArray(0);
    glBindBuffer(GL_ARRAY_BUFFER, 0);

    while (!glfwWindowShouldClose(glwindow)) {
        glfwPollEvents();
        processInput(glwindow);

        glClearColor(0.2f, 0.3f, 0.3f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        glm_rotate(idenMat4, glm_rad(1), (vec3){0, 1, 1});
        glUniformMatrix4fv(glGetUniformLocation(shaderProgram, "transform"), 1,
                           false, idenMat4[0]);

        // bind our triangle
        glBindVertexArray(vao);
        // draw the triangle
        glDrawArrays(GL_TRIANGLES, 0, 3);

        glfwSwapBuffers(glwindow);
    }

    glfwTerminate();
    return 0;
}
