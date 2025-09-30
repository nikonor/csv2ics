# cvs2ics

A simple utility for converting a CSV file to an ICS file.

You can create a CVS file with two columns (title and description) and run this utility. The CSV file will be converted to an ICS file. The first CSV record will be converted to an event on the first monday date, and the second record will be converted to an event on the day after the first monday date.

Usage:

> Usage: cvs2ics <first monday date> <file.csv> <file.ics>
