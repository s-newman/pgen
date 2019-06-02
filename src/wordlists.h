/**
 * @file wordlists.h
 * @author Sean Newman (sxn6296@rit.edu)
 * @brief Functions and definitions to manage the program's wordlists.
 * @version 0.1
 * @date 2019-06-02
 * 
 * @copyright Copyright (c) 2019
 * 
 */

#ifndef NEWMAN_WORDLISTS_H
#define NEWMAN_WORDLISTS_H

#include <string>
#include <vector>

/**
 * @brief An abstraction of the available wordlists.
 * 
 */
enum Wordlist {
    FULL,
    SHORT,
    NUMBERS,
    SYMBOLS
};

/**
 * @brief The start of a wordlist containing words no longer than 7 characters.
 * 
 */
extern char _binary_wordlists_short_words_txt_start;

/**
 * @brief The end of a wordlist containing words no longer than 7 characters.
 * 
 */
extern char _binary_wordlists_short_words_txt_end;

/**
 * @brief The start of a wordlist containing approximately 370,000 words.
 * 
 */
extern char _binary_wordlists_370k_words_txt_start;

/**
 * @brief The end of a wordlist containing approximately 370,000 words.
 * 
 */
extern char _binary_wordlists_370k_words_txt_end;

/**
 * @brief The start of a wordlist containing only the digits 0 through 9.
 * 
 */
extern char _binary_wordlists_numbers_txt_start;

/**
 * @brief The end of a wordlist containing only the digits 0 through 9.
 * 
 */
extern char _binary_wordlists_numbers_txt_end;

/**
 * @brief The start of a wordlist containing only special characters.
 * 
 */
extern char _binary_wordlists_symbols_txt_start;

/**
 * @brief The end of a wordlist containing only special characters.
 * 
 */
extern char _binary_wordlists_symbols_txt_end;

/**
 * @brief Read a specified wordlist from memory.
 * 
 * @param list The wordlist to be read.
 * @return std::vector<std::string> A list of the words in the wordlist.
 */
std::vector<std::string> get_wordlist(enum Wordlist list);

#endif