
You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input).

The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.

Meaning of Terminal input

- lines that begin with $ are commands you executed

- cd means change directory
  - cd x  => moves in one level  
  - cd .. => moves out one level
  - cd /  => switch to outermost dir '/'
  
- ls means list. It prints out all of the files and directories immediately contained by the current directory:
  - 123 abc == <size> <filename>
  - dir xyz == dir <dirname>

# Part 1

Disk is full => need to find delete good deletion candidate directiories :
    - According to total size of dir
  
Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?