/**
 * @file random_help.h
 * @author Sean Newman (sxn6296@rit.edu)
 * @brief Wrapper functions for the OpenSSL random library.
 * @version 0.1
 * @date 2019-06-02
 * 
 * @copyright Copyright (c) 2019
 * 
 */

#ifndef NEWMAN_RANDOM_HELP_H
#define NEWMAN_RANDOM_HELP_H

/**
 * @brief Initialize the pseudo-random number generator.
 * 
 * @return true when the initialization was successful.
 * @return false when the initialization failed for some reason.
 */
bool random_init();

/**
 * @brief Get a random integer greater than zero and less than a specified
 *      upper bound.
 * 
 * @param upper_bound The exclusive upper bound on the possible range of
 *      numbers that could be generated.
 * @return unsigned long The randomly-generated number, or ULONG_MAX if there
 *      was an error generating the number.
 */
unsigned long random_int(unsigned long upper_bound);

#endif