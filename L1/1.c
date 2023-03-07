#include<stdio.h>
#include<stdlib.h>
#include<assert.h>
typedef struct{
    size_t head;
    size_t tail;
    size_t size;
    void** data;
}fifo_queue;

void* fifo_read(fifo_queue *queue){
    if(queue->tail == queue->head){
        return NULL;
    }
    void* handle = queue->data[queue->tail];
    queue->data[queue->tail] = NULL;
    queue->tail = (queue->tail + 1) % queue->size;
    return handle;
}

void* fifo_write(fifo_queue *queue, void* handle){
    if(((queue->head + 1) % queue->size) ==  queue->tail){
        return -1;
    }
    queue->data[queue->head] = handle;
    queue->head = (queue->head + 1) % queue->size;
    return 0;
}

fifo_queue make_fifo(int size){
    fifo_queue queue = {0, 0, size, malloc(sizeof(void*) * size)};
    return queue;
}
