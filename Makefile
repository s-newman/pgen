# The name of the final executable binary.
MAIN := pgen

# The compiler to use.
CC := g++

# Generic compiler flags.
CFLAGS := -Wall -Wextra -Werror -pedantic -ansi

# Additional directories to look in for header files.
#INCLUDES = -I/home/user/include
INCLUDES :=

# Additional directories to look in for libraries.
#LFLAGS = -L/home/user/include
LFLAGS :=

# Libraries to link into the executable.
# LIBS = -lpthread
LIBS :=

# The directory to look in for source code files.
SRCDIR := src

# The directory in which all build artifacts except the final executable binary
# will be placed.
BUILDDIR := build

# The directory to place dependency files in.
DEPDIR := $(BUILDDIR)

# File extension to use for source files.
SRCEXT := cpp

# File extension to use for dependency files.
DEPEXT := depend

# File extension to use for temporary dependency files.
TMPDEPEXT := $(DEPEXT).tmp

# File extension to use for object files.
OBJEXT := o

# Compiler flags related to dependency generation.
DEPFLAGS = -MT $@ -MMD -MP -MF $(DEPDIR)/$*.$(TMPDEPEXT)

# Find all source code files.
SRCS := $(wildcard *.$(SRCEXT)) $(wildcard **/*.$(SRCEXT))

# Get the list of object files, with paths, from the list of source files.
OBJS := $(patsubst %.$(SRCEXT),%.$(OBJEXT),$(SRCS))

# Renames temporary dependency files to real dependency files, then touches the
# object file. This makes sure that the object file is newer than the real
# dependency file so make doesn't have to rebuild everything every time.
POSTCOMPILE = @mv -f $(DEPDIR)/$*.$(TMPDEPEXT) $(DEPDIR)/$*.$(DEPEXT) && \
	touch $@

# Make sure BUILDDIR and DEPDIR are created.
$(shell mkdir -p $(BUILDDIR) $(DEPDIR) > /dev/null)

.PHONY: clean

all: $(MAIN)
	@echo Program $(MAIN) has been compiled.

$(MAIN): $(BUILDDIR)/$(OBJS)
	$(CC) $(CFLAGS) $(INCLUDES) -o $(MAIN) $(BUILDDIR)/$(OBJS) $(LFLAGS) $(LIBS)

# Generic rule for building object files from source files.
$(BUILDDIR)/%.$(OBJEXT) : %.$(SRCEXT) $(DEPDIR)/%.$(DEPEXT)
	# Put other flags here too
	@mkdir -p $(@D)
	$(CC) $(DEPFLAGS) -c $< -o $@
	$(POSTCOMPILE)

clean:
	$(RM) -rf *~ $(BUILDDIR) $(MAIN)

# Ensure make doesn't fail if a dependency file doesn't exist.
$(DEPDIR)/%.$(DEPEXT): ;

# Ensure make doesn't automatically delete dependency files.
.PRECIOUS: $(DEPDIR)/%.$(DEPEXT)

# Include all existing dependency files.
include $(wildcard $(patsubst %,$(DEPDIR)/%.$(DEPEXT),$(basename $(SRCS))))