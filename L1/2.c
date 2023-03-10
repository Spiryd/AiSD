#include<stdio.h>
#include<stdlib.h>
#include<time.h>

typedef struct node
{
    int value;
    struct node* next_node;
}node_t;

void print_list(node_t* head){
    node_t* tmp_node =  head;
    while (tmp_node != NULL)
    {
        printf("%d\n", tmp_node->value);
        tmp_node = tmp_node->next_node;
    }
}

void append(node_t* head, int value){
    node_t* tmp_node =  head;
    while (tmp_node->next_node != NULL)
    {
        tmp_node = tmp_node->next_node;
    }
    tmp_node->next_node = (node_t*) malloc(sizeof(node_t));
    tmp_node->next_node->value = value;
    tmp_node->next_node->next_node = NULL;
}

void merge(node_t* head1, node_t* head2){
    node_t* tmp_node =  head1;
    while (tmp_node->next_node != NULL)
    {
        tmp_node = tmp_node->next_node;
    }
    tmp_node->next_node = head2;
}

int getById(node_t* head, int id){
    int count = 0;
    node_t* tmp_node =  head;
    while (count != id || tmp_node != NULL)
    {
        tmp_node = tmp_node->next_node;
        count++;
    }
    if (tmp_node == NULL){
        printf("something went wrong");
        return 0;
    }else
    {
        return tmp_node->value;
    }
}

int main(){
    srand(time(NULL));

    node_t* head = NULL;
    head = (node_t*) malloc(sizeof(node_t));
    if(head == NULL){
        return 1;
    }
    head->value = rand();
    head->next_node = NULL;

    for (size_t i = 0; i < 10000; i++)
    {
        append(head, rand());
    }

    double time[2000];
    clock_t t;
    for (size_t i = 0; i < 2000; i++)
    {
        t = clock();
        getById(head, 5);
        t = clock() - t;
        time[i] = ((double)t) / CLOCKS_PER_SEC;
    }

    for (size_t i = 0; i < 2000; i++)
    {
        printf("%lf\n", time[i]);
    }

    return 0;
}
