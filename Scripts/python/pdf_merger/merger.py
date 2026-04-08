from PyPDF2 import PdfMerger

# Create a PdfMerger object
merger = PdfMerger()

# Append the PDFs
merger.append('file1.pdf')
merger.append('file2.pdf')

# Write out the merged PDF
merger.write('merged.pdf')
merger.close()

