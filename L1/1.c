#include<stdio.h>
#include<stdlib.h>
#include<assert.h>

//classic fifo queue
typedef struct{
    size_t head;
    size_t tail;
    size_t size;
    void** data;
}fifo;

//read from fifo queue
void* fifo_read(fifo *queue){
    if(queue->tail == queue->head){
        return NULL;
    }
    void* handle = queue->data[queue->tail];
    queue->data[queue->tail] = NULL;
    queue->tail = (queue->tail + 1) % queue->size;
    return handle;
}

//write to fifo queue
void* fifo_write(fifo *queue, void* handle){
    if(((queue->head + 1) % queue->size) ==  queue->tail){
        return -1;
    }
    queue->data[queue->head] = handle;
    queue->head = (queue->head + 1) % queue->size;
    return 0;
}

//construckt empty fifo queue
fifo make_fifo(int size){
    fifo queue = {0, 0, size, malloc(sizeof(void*) * size)};
    return queue;
}

//classic stack(FILO)
typedef struct{
    size_t top;
    size_t size;
    void** data;
}stack;

//checs if stack is full
int is_full(stack *st){
    if(st->top == st->size - 1){
        return 1;
    }else{
        return 0;
    }
}

//checs if stack is empty
int is_empty(stack *st){
    if(st->top == -1){
        return 1;
    }else{
        return 0;
    }
}

//pushes data on top of a stack
void push(stack *st, void* data){
    if(is_full(st)){
        return -1;
    }else
    {
        st->top++;
        st->data[st->top] = data; 
    }
}

//pops off the top of the stack
void* pop(stack *st){
    if(is_empty(st)){
        return NULL;
    }else
    {
        void* data =  st->data[st->top];
        st->top--;
        return data;
    }
  
}

//makes an empty stack
stack make_stack(int size){
    stack st = {0, size, malloc(sizeof(void*) * size)};
    return st;
}
