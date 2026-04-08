import pandas as pd
from matplotlib.patches import Rectangle
import matplotlib.pyplot as plt
import matplotlib.font_manager as fm
from matplotlib.colors import to_rgba

# TODO: Modify this to properly set the columns and rows

# Load the user's CSV file to inspect its structure
file_path = './1000_korean_words.csv'
vocab_data = pd.read_csv(file_path)

# Define parameters for the image
background_color = to_rgba("black")
text_color = "black"
font_size = 14 
font = fm.FontProperties(fname='./fonts/NotoSerifKR-VariableFont_wght.ttf')
dpi = 100  # Higher DPI for clarity

# Extract and format the data for visualization
data = vocab_data.fillna("").values  # Ensure no NaN values
data_flat = data.flatten()
data_text = "\n".join(data_flat)  # Combine all rows into a single text block


# Calculate figure size for a 2560x1440 image
fig_width, fig_height = 2560 / dpi, 1440 / dpi

# Create the image
plt.figure(figsize=(fig_width, fig_height), dpi=dpi)
plt.text(0.5, 0.5, data_text, fontsize=font_size, color=text_color, ha="center", va="center", wrap=True, fontproperties=font)

# Style adjustments
plt.gca().set_facecolor(background_color)
plt.axis("off")

# Save the image
output_path = "./out/korean_vocab_background.png"
plt.savefig(output_path, bbox_inches="tight", dpi=dpi)
plt.close()





# Prepare data: split into manageable rows and columns
rows, cols = data.shape

# Calculate layout: number of rows and columns in the image grid
max_cols = 16  # Number of word pairs per line to improve readability
font_size = 14

# Create the image
fig, ax = plt.subplots(figsize=(2560 / 100, 1440 / 100), dpi=100)
ax.set_facecolor("black")
ax.set_xlim(0, max_cols)
ax.set_ylim(0, len(data))
ax.axis("off")

# Draw text
for i, row in enumerate(data):
    for j, cell in enumerate(row):
        if cell:  # Skip empty cells
            x_pos = j % max_cols
            y_pos = len(data) - i - 1  # Reverse for top-to-bottom text
            ax.text(
                x_pos + 0.1, y_pos - 0.5, cell, 
                fontsize=font_size, color="black", 
                va="center", ha="left", fontproperties=font
            )

# Save the image
output_path_fixed = "./out/korean_vocab_background_grid.png"
plt.savefig(output_path_fixed, bbox_inches="tight", dpi=100)
plt.close()




