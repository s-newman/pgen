#include <iostream>
#include <iterator>
#include <sstream>
#include <vector>

#include <openssl/err.h>
#include <openssl/rand.h>

#include "random_help.h"
#include "wordlists.h"

int main(void)
{
    enum Wordlist param = FULL;
    std::vector<std::string> wordlist = get_wordlist(param);

    if(!random_init())
    {
        return EXIT_FAILURE;
    }

    for(int idx = 0; idx < 4; idx++)
    {
        std::cout << wordlist[random_int(wordlist.size())];
    }
    std::cout << std::endl;

    return EXIT_SUCCESS;
}