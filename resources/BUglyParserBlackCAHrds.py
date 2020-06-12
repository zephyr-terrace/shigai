# John Gallagher
# June 12, 2020
#  
# This is a program for parsing the txt file that I copy pasted from the Cards Against Humanity pdf.  
# The black cards have a lot of white-space and periods, and weird lines. 
# All text between periods is on one line. 
# New line after every period.  
# Hopefully this works... it's been a while... O_O
#
# Structure of the program:
# 1. Function to Input file
# 2. Function to put everything on the same line
# 3. Function to remove whitespace after every period and before any letter
# 4. Function to make a new line after every period. 
# For more info on the regex used here, I have a worked example with explanation. https://regex101.com/r/aNXuLN/1
# 5. Write file
# Function to input file, do 1,2,3,4, then output a file. 
# 
import os    
import re

# Make sure we're working in the right place
MyFileLoc = os.environ['BLACKCAHDSLOCATION']
os.chdir(MyFileLoc)

# the function:
def parseCAHds(fileIN, fileOUT):
    with open(fileIN, encoding='utf-8', errors='ignore') as f:
            #1
            read_data = f.read()
            #2
            read_data = read_data.replace('\n', ' ')
            #3 Find any periods, except for middle initials (capital letter + \.),  
            regex = r"(?<![A-Z\.])\.(\s+)"
            #4 Then replace with '.\newline'
            subst = ".\\n"
            # Do 3,4
            read_data = re.sub(regex, subst, read_data, 0, re.MULTILINE)
            #5 write file
            with open(fileOUT, 'w') as output:
                    output.write(read_data)

#now actually do the job:
parseCAHds('blackCAHrdsRAW.txt', 'blackCAHdsParsed.txt')