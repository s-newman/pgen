/**
 * @file wordlists.cpp
 * @author Sean Newman (sxn6296@rit.edu)
 * @brief Functions and definitions to manage the program's wordlists.
 * @version 0.1
 * @date 2019-06-02
 * 
 * @copyright Copyright (c) 2019
 * 
 */

#include "wordlists.h"

#include <iterator>
#include <sstream>
#include <vector>

/**
 * @brief A helper function to actually load the wordlist from memory.
 * 
 * @param start The address of the start of the wordlist.
 * @param end The address of the end of the wordlist.
 * @return std::vector<std::string> A list of the words in the wordlist.
 */
std::vector<std::string> _load_wordlist(char *start, char *end)
{
    // Use a temporary variable to store all the characters in the wordlist
    std::string wordlist_chars = "";
    char *current_addr = start;

    // Load all the characters from memory
    while(current_addr < end)
    {
        wordlist_chars += *current_addr;
        current_addr++;
    }

    // Convert the continuous string into a vector of strings/words
    std::istringstream iss(wordlist_chars);
    std::vector<std::string> results(
        (std::istream_iterator<std::string>(iss)),
        std::istream_iterator<std::string>()
    );

    return results;
}

std::vector<std::string> get_wordlist(enum Wordlist list)
{
    char *start;
    char *end;

    // Determine which wordlist to use
    switch(list){
        case FULL:
            start = &_binary_wordlists_370k_words_txt_start;
            end = &_binary_wordlists_370k_words_txt_end;
            break;
        
        case SHORT:
            start = &_binary_wordlists_short_words_txt_start;
            end = &_binary_wordlists_short_words_txt_end;
            break;
        
        case NUMBERS:
            start = &_binary_wordlists_numbers_txt_start;
            end = &_binary_wordlists_numbers_txt_end;
            break;
        
        case SYMBOLS:
            start = &_binary_wordlists_symbols_txt_start;
            end = &_binary_wordlists_symbols_txt_end;
            break;
        
        default:
            start = &_binary_wordlists_short_words_txt_start;
            end = &_binary_wordlists_short_words_txt_end;
    }

    // Call the helper function
    return _load_wordlist(start, end);
}