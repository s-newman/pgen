#include <iostream>
#include <iterator>
#include <sstream>
#include <vector>

#include <openssl/err.h>
#include <openssl/rand.h>

extern char _binary_wordlists_short_words_txt_start;
extern char _binary_wordlists_short_words_txt_end;

std::vector<std::string> load_words()
{
    std::string wordlist_chars = "";

    // Read the wordlist contents from memory
    char *file = &_binary_wordlists_short_words_txt_start;
    while(file < &_binary_wordlists_short_words_txt_end)
    {
        wordlist_chars += *file;
        file++;
    }

    // Convert the string to a vector of words
    std::istringstream iss(wordlist_chars);
    std::vector<std::string> results(
        (std::istream_iterator<std::string>(iss)),
        std::istream_iterator<std::string>()
    );

    return results;
}

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
    std::vector<std::string> wordlist = load_words();

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