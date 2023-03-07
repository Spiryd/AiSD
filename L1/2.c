#include<stdio.h>
#include<stdlib.h>

typedef struct
{
    int value;
    node* next_node;
}node;

void print_list(node* head){
    node* tmp_node =  head;
    while (tmp_node->next_node != NULL)
    {
        printf("%d", tmp_node->value);
        tmp_node = tmp_node->next_node;
    }
}

void append(node* head, int value){
    node* tmp_node =  head;
    while (tmp_node->next_node != NULL)
    {
        tmp_node = tmp_node->next_node;
    }
    tmp_node->next_node = (node*) malloc(sizeof(node));
    tmp_node->next_node->value = value;
    tmp_node->next_node->next_node = NULL;
}

void merge(node* head1, node* head2){
    node* tmp_node =  head1;
    while (tmp_node->next_node != NULL)
    {
        tmp_node = tmp_node->next_node;
    }
    tmp_node->next_node = head2;
}

int getById(node* head, int id){
    int count = 0;
    node* tmp_node =  head;
    while (count != id || tmp_node != NULL)
    {
        tmp_node = tmp_node->next_node;
        count++;
    }
    if (tmp_node == NULL){
        printf("something went wrong");
        return NULL;
    }else
    {
        return tmp_node->value;
    }
    
}

int main(){
    return 0;
}