#include <vector>
#include <cstdio>

int main()
{
    std::vector<unsigned> *x;
    {
        std::vector<unsigned> y = {1,2,3,4,5};
        x = &y;
    }
    printf("x's length is {}", x->size());  // allowed! 💥
}
