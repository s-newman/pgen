/**
 * @file random_help.cpp
 * @author Sean Newman (sxn6296@rit.edu)
 * @brief Wrapper functions for the OpenSSL random library.
 * @version 0.1
 * @date 2019-06-02
 * 
 * @copyright Copyright (c) 2019
 * 
 */

#include "random_help.h"

#include <iostream>

#include <openssl/err.h>
#include <openssl/rand.h>

bool random_init()
{
    // Initialize using /dev/random
    int retcode = RAND_load_file("/dev/random", 32);

    // Check the error code
    if(retcode != 32)
    {
        std::cerr << "ERROR: failed to seed the PRNG with /dev/random" << \
            std::endl;
        return false;
    }
    
    // We succeeded
    return true;
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