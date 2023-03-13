#include <cstdlib>
#include <iostream>
#include <time.h>
#include <chrono>

using namespace std;
using namespace std::chrono;

struct Node { 
  int value; 
  Node *next;
  // Parameterised constructor with default argument
  Node(int val=0) :value(val),next(nullptr){}
  // Parameterise constructor
  Node(int val, Node *tempNext):value(val),next(tempNext){}
};

class List
{
private:
    Node *head;
public:
    List():head(nullptr){}

    ~List(){}

    void append(int val){
        Node *new_node = new Node(val);
        if(head == nullptr){
            head = new_node;
        }else {      
            Node *tmp = head;  
            while (tmp->next != nullptr)
            {
                tmp = tmp->next;
            }
            tmp->next = new_node;
        }
    }

    void print(){
        Node* tmp = head;
        while(tmp != nullptr)
        {
          cout << tmp->value << " ";
          tmp = tmp->next;
        }
        cout << endl;
    }

    int getById(int id){
        int count = 0;
        Node* tmp = head;
        while (count !=  id)
        {
            tmp = tmp->next;
            count++;
        }
        return tmp->value;
    }

    void merge(List* list){
        Node* tmp = head;
        while (tmp->next != nullptr)
        {
            tmp = tmp->next;
        }
        tmp->next = list->head;
        
    }
};

int main(){
    srand(time(0));
    cout << "Tworze liste randomowych int:" << endl;
    List list;
    for (size_t i = 0; i < 10000; i++)
    {
        list.append(rand());
    }
    list.print();
    int sum = 0;
    for (size_t i = 0; i < 1000; i++)
    {
        auto start = high_resolution_clock::now();
        list.getById(999);
        auto stop = high_resolution_clock::now();
        auto duration  = duration_cast<microseconds>(stop - start);
        //cout << duration.count() << endl;
        sum += duration.count();
    }
    cout << "Avg. time accesing the 1000th element: " << sum/1000 << " microseconds" << endl;
    sum = 0;
    for (size_t i = 0; i < 1000; i++)
    {
        auto start = high_resolution_clock::now();
        list.getById(7999);
        auto stop = high_resolution_clock::now();
        auto duration  = duration_cast<microseconds>(stop - start);
        //cout << duration.count() << endl;
        sum += duration.count();
    }
    cout << "Avg. time accesing the 8000th element: " << sum/1000 << " microseconds" << endl;
    sum = 0;
    for (size_t i = 0; i < 1000; i++)
    {
        auto start = high_resolution_clock::now();
        list.getById(rand() % 9999);
        auto stop = high_resolution_clock::now();
        auto duration  = duration_cast<microseconds>(stop - start);
        //cout << duration.count() << endl;
        sum += duration.count();
    }
    cout << "Avg. time accesing the random element: " << sum/1000 << " microseconds" << endl;
    cout << "Tworze liste 1:" << endl;
    List list1;
    list1.append(1);
    list1.append(2);
    list1.append(3);
    list1.append(4);
    list1.print();
    cout << "Tworze liste 2:" << endl;
    List list2;
    list2.append(1);
    list2.append(2);
    list2.append(3);
    list2.append(4);
    list2.print();
    cout << "Łącze listy" << endl;
    list1.merge(&list2);
    list1.print();
    return 0;
}

