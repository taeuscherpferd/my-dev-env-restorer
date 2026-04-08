import math
import os
import sys
from pdf2image import convert_from_path
from PIL import Image

def extract_and_rotate_quadrants(
        page_image: Image.Image,
        left: float,
        top: float,
        quadrant_width: float,
        quadrant_height: float
        ):
    """
    Given a PIL image of a page (page_image), crop out the rectangle whose
    top-left corner is (left, top) and whose total size is
    2*quadrant_width by 2*quadrant_height.

    Then subdivide that rectangle into 4 quadrants and rotate each 90° CCW.
    Returns the 4 quadrants as a tuple (q1, q2, q3, q4).
    """

    # Compute the right/bottom edges of the entire bounding rectangle
    # we want to extract.
    right  = left + 2 * quadrant_width
    bottom = top  + 2 * quadrant_height

    # 1) Crop out the big rectangle from the page
    cropped = page_image.crop((left, top, right, bottom))

    # 2) Extract the four quadrants.
    #    Quadrant layout (before rotation):
    #       +---------+---------+
    #       |   Q1    |   Q2    |
    #       +---------+---------+
    #       |   Q3    |   Q4    |
    #       +---------+---------+
    #
    #    Each quadrant is quadrant_width wide, quadrant_height high.

    # Q1: top-left
    q1 = cropped.crop((0,
                       0,
                       quadrant_width,
                       quadrant_height))
    # Q2: top-right
    q2 = cropped.crop((quadrant_width,
                       0,
                       2 * quadrant_width,
                       quadrant_height))
    # Q3: bottom-left
    q3 = cropped.crop((0,
                       quadrant_height,
                       quadrant_width,
                       2 * quadrant_height))
    # Q4: bottom-right
    q4 = cropped.crop((quadrant_width,
                       quadrant_height,
                       2 * quadrant_width,
                       2 * quadrant_height))

    # 3) Rotate each quadrant 90° to the left (counterclockwise).
    #    PIL’s rotate(90) rotates anticlockwise by default; `expand=True`
    #    ensures the resulting image canvas fits the entire rotated quadrant.
    q1 = q1.rotate(90, expand=True)
    q2 = q2.rotate(90, expand=True)
    q3 = q3.rotate(90, expand=True)
    q4 = q4.rotate(90, expand=True)

    return q1, q2, q3, q4

def process_pdf(
        pdf_path: str,
        output_prefix: str = "output",
        dpi: int = 72,
        # Adjust these to match the bounding rectangle's location on the page:
        bounding_left: float   = 100.0,
        bounding_top: float    = 100.0,
        # The final quadrant must be 453.7 wide × 738 high in 'points'
        # If you render at 72 DPI, 1 PDF point = 1 pixel.  If you use 300 DPI,
        # multiply these by (300/72) etc.
        quadrant_width: float  = 453.7,
        quadrant_height: float = 738.0
        ):
    """
    Main function to:
      1) convert a PDF into images
      2) for pages i and i+1 (0-indexed, so that’s pages (1,2), (3,4), etc.),
         extract 4 rotated quadrants from each
      3) name and save each quadrant
    """

    # 1) Convert the PDF into a list of PIL images (one per page).
    #    You can pick the DPI you prefer. 72 DPI is the simplest for 1:1 point scaling.
    page_images = convert_from_path(pdf_path, dpi=dpi)

    # 2) Process the pages in pairs: (0,1), (2,3), (4,5), ...
    for i in range(0, len(page_images), 2):
        page_num_1 = i + 1        # 1-based page numbering for naming
        page_num_2 = i + 2
        img_page_1 = page_images[i]

        # For odd total number of pages, the last “pair” might just have a single page:
        img_page_2 = page_images[i+1] if (i + 1) < len(page_images) else None

        # Extract 4 quadrants from page 1:
        p1q1, p1q2, p1q3, p1q4 = extract_and_rotate_quadrants(
                img_page_1,
                bounding_left,
                bounding_top,
                quadrant_width,
                quadrant_height
                )

        # If there *is* a page 2 in this pair, do the same:
        if img_page_2 is not None:
            p2q1, p2q2, p2q3, p2q4 = extract_and_rotate_quadrants(
                    img_page_2,
                    bounding_left,
                    bounding_top,
                    quadrant_width,
                    quadrant_height
                    )
        else:
            # If there's no second page in the pair, just set them to None
            p2q1 = p2q2 = p2q3 = p2q4 = None

        if not os.path.exists("./output"):
            os.makedirs("output")

        # Now we have Page1 Q1–Q4, and Page2 Q1–Q4.
        # The “linking” is:  (page1q1 - page2q2), (page1q2 - page2q1),
        #                    (page1q3 - page2q4), (page1q4 - page2q3).
        #
        # You mentioned each quadrant itself should be saved at 453.7 × 738.
        # We'll just save each quadrant as its own PNG.  If you need to pair them
        # in some combined image, you can do that with a separate step.

        # For demonstration, we’ll show how to name them in a way that indicates the link:
        # e.g.   output_page01_q1.png  etc.

        # Save Page1 quadrants
        p1q1.save(f"{output_prefix}_page{page_num_1}_q1.png")
        p1q2.save(f"{output_prefix}_page{page_num_1}_q2.png")
        p1q3.save(f"{output_prefix}_page{page_num_1}_q3.png")
        p1q4.save(f"{output_prefix}_page{page_num_1}_q4.png")

        # Save Page2 quadrants if they exist
        if p2q1 is not None:
            p2q1.save(f"{output_prefix}_page{page_num_2}_q1.png")
            p2q2.save(f"{output_prefix}_page{page_num_2}_q2.png")
            p2q3.save(f"{output_prefix}_page{page_num_2}_q3.png")
            p2q4.save(f"{output_prefix}_page{page_num_2}_q4.png")

        # If you want to highlight how Q1 from page1 is “linked” to Q2 from page2, etc.,
        # you could *also* rename or do some pairing. For now we just save each quadrant
        # individually.

if __name__ == "__main__":

    if len(sys.argv) < 2:
        print("Usage: python win_card_splicer.py <PDF_PATH>")
        sys.exit(1)

    input_pdf_path = sys.argv[1]
    # Example usage:
    process_pdf(
            pdf_path=input_pdf_path,
            output_prefix="output/cropped_quadrants",
            dpi=300,
            bounding_left=79 * (300/72),
            bounding_top=27.1 * (300/72),
            quadrant_width=(453.7 / 2) * (300/72),
            quadrant_height=(738.0 / 2) * (300/72)
            )

