#include<stdio.h>
#include<stdlib.h>

typedef struct
{
    node* prev_node;
    int value;
    node* next_node;
}node;

void append(node* head, int value){
    node* tmp_node = head;
    while (tmp_node->next_node != head)
    {
       tmp_node = tmp_node->next_node;
    }
    node* new_node = (node*) malloc(sizeof(node));
    tmp_node->next_node = new_node;
    new_node->prev_node = tmp_node;
    new_node->next_node = head;
    new_node->value = value;  
}

void print_list(node* head){
    node* tmp_node = head;
    while (tmp_node->next_node != head)
    {
        printf("%d", tmp_node->value);
        tmp_node = tmp_node->next_node;
    }
}

int getById(node* head, int id){
    node* tmp_node = head;
    for (size_t i = 0; i < id; i++)
    {
        tmp_node = tmp_node->next_node;
    }
    return tmp_node->value;
}

void merge(node* head1,node* head2){
    node* tmp_node = head1;
    while (tmp_node->next_node != head1)
    {
        tmp_node = tmp_node->next_node;
    }
    tmp_node->next_node = head2;
    head2->prev_node = tmp_node;
    tmp_node = head2;
    while (tmp_node->next_node != head2)
    {
        tmp_node = tmp_node->next_node;
    }
    tmp_node->next_node = head1;
    head1->prev_node = tmp_node;
}
