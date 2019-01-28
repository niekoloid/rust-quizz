#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

enum METHOD { GET, POST };

typedef struct _Request {
    enum METHOD method;
    char* path;
    char* query;
    char* request_body;
} Request;

enum METHOD request_get_method(Request* req) { return req->method; }
char* request_get_path(Request* req) { return req->path; }
char* request_get_query(Request* req) { return req->query; }
char* request_get_request_body(Request* req) { return req->request_body; }

typedef struct _Response {
    int stop;
    int status;
    char* response_body;
} Response;
void response_set_status(Response* response, int status) {
    response->status = status;
}
void response_set_response_body(Response* response, char* body) {
    response->response_body = body;
}
void response_set_stop(Response* response, int stop) {
    response->stop = stop;
}

typedef int (*RequestCallback)(Request* request, Response* response);

typedef struct _Context {
    RequestCallback callback;
} Context;

Context* new_context() { return (Context*)malloc(sizeof(Context)); }

void context_set_callback(Context* context, RequestCallback callback) {
    context->callback = callback;
}

int send_msg(int fd, char* msg) {
    int len = strlen(msg);
    if (write(fd, msg, len) != len) {
        fprintf(stderr, "error: writing.");
    }
    return len;
}

// Hello World API
int serve(Context* context) {
    int ss;
    struct sockaddr_in addr;

    ss = socket(AF_INET, SOCK_STREAM, 0);
    int enable = 1;
    setsockopt(ss, SOL_SOCKET, SO_REUSEADDR, &enable, sizeof(int));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(8080);
    addr.sin_addr.s_addr = INADDR_ANY;

    bind(ss, (struct sockaddr*)&addr, sizeof(addr));

    listen(ss, 5);

    for (;;) {
        char path[128];
        struct sockaddr_in client;
        int ret;
        char buf[1024];

        unsigned int len = sizeof(client);
        int cs = accept(ss, (struct sockaddr*)&client, &len);

        printf("Accept\n");
        Response response;
        response.status = 200;

        // test for debugging
        response.response_body = "test\n";

        while ((ret = read(cs, buf, 1024)) > 0) {
            char* end = buf + ret - 4;
            buf[ret] = '\0';

            if (*end++ == '\r' && *end++ == '\n' && *end++ == '\r' &&
                *end++ == '\n') {
                char* req_str;
                if ((req_str = strstr(buf, "GET ")) != NULL) {
                    for (int i = 4, j = 0; i < 124 && req_str[i] != '\0';
                         i++, j++)
                        path[j] = req_str[i];
                    *(strstr(path, " HTTP")) = '\0';
                    Request request;
                    request.method = GET;
                    request.path = path;
                    request.query = path;
                    request.request_body = req_str;
                    context->callback(&request, &response);
                }
                if ((req_str = strstr(buf, "POST ")) != NULL) {
                    for (int i = 5, j = 0; i < 124 && req_str[i] != '\0';
                         i++, j++)
                        path[j] = req_str[i];
                    *(strstr(path, " HTTP")) = '\0';
                    Request request;
                    request.method = POST;
                    request.path = path;
                    request.query = "";
                    request.request_body = req_str;
                    context->callback(&request, &response);
                }
                break;
            }
        }

        printf("GET PATH = '%s'\n", path);

        char status[32];
        sprintf(status, "HTTP/1.0 %d OK\r\n", response.status);
        send_msg(cs, status);
        send_msg(cs, "Content-Type: text/html\r\n");
        send_msg(cs, "\r\n");
        send_msg(cs, response.response_body);
        close(cs);

        printf("CLOSE\n");

        if (response.stop) {
            break;
        }
    }

    printf("SHUTDOWN SERVER\n");
    close(ss);
    return 0;
}
