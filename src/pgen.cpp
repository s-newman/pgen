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
    enum Wordlist param = SHORT;
    std::vector<std::string> wordlist = get_wordlist(param);
    param = NUMBERS;
    std::vector<std::string> numblist = get_wordlist(param);
    param = SYMBOLS;
    std::vector<std::string> symblist = get_wordlist(param);
    std::vector<std::string> charlist = numblist;
    charlist.insert(charlist.end(), symblist.begin(), symblist.end());

    if(!random_init())
    {
        return EXIT_FAILURE;
    }

    for(int idx = 0; idx < 3; idx++)
    {
        std::cout << wordlist[random_int(wordlist.size())];
        std::cout << charlist[random_int(charlist.size())];
    }
    std::cout << std::endl;

    return EXIT_SUCCESS;
}