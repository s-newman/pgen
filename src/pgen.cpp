#include <iostream>
#include <iterator>
#include <sstream>
#include <vector>

#include <openssl/err.h>
#include <openssl/rand.h>

#include "wordlists.h"

unsigned long random_int(unsigned long upper_bound)
{
    // Get the random number
    unsigned long int result = ULONG_MAX;
    int retcode = RAND_bytes((unsigned char *)&result, sizeof(unsigned long));

    // Check the error code
    if(retcode != 1)
    {
        unsigned long errcode = ERR_get_error();
        char errtext[256];
        ERR_error_string(errcode, errtext);
        std::cerr << "ERROR: failed to get random number - OpenSSL " << \
            "returned error: " << errtext << std::endl;
        return ULONG_MAX;
    }
    else
    {
        return result % upper_bound;
    }
}

int main(void)
{
    std::vector<std::string> wordlist = get_wordlist(Wordlist::SHORT);

    // Re-seed the PRNG
    int retcode = RAND_load_file("/dev/random", 32);
    if(retcode != 32)
    {
        std::cerr << "ERROR: failed to seed the PRNG with /dev/random" << \
            std::endl;
        return EXIT_FAILURE;
    }

    for(int idx = 0; idx < 4; idx++)
    {
        std::cout << wordlist[random_int(wordlist.size())];
    }
    std::cout << std::endl;

    return EXIT_SUCCESS;
}