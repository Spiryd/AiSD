#include<stdio.h>
#include<stdlib.h>

typedef struct node
{
    struct node* prev_node;
    int value;
    struct node* next_node;
}node_t;

void append(node_t* head, int value){
    node_t* tmp_node = head;
    while (tmp_node->next_node != head)
    {
       tmp_node = tmp_node->next_node;
    }
    node_t* new_node = (node_t*) malloc(sizeof(node_t));
    tmp_node->next_node = new_node;
    new_node->prev_node = tmp_node;
    new_node->next_node = head;
    new_node->value = value;  
}

void print_list(node_t* head){
    node_t* tmp_node = head;
    while (tmp_node->next_node != head)
    {
        printf("%d", tmp_node->value);
        tmp_node = tmp_node->next_node;
    }
}

int getById(node_t* head, int id){
    node_t* tmp_node = head;
    for (size_t i = 0; i < id; i++)
    {
        tmp_node = tmp_node->next_node;
    }
    return tmp_node->value;
}

void merge(node_t* head1,node_t* head2){
    node_t* tmp_node = head1;
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

int main(){
    return 0;
}
