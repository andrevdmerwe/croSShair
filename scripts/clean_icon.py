from PIL import Image
import sys

def remove_background(input_path, output_path):
    img = Image.open(input_path).convert("RGBA")
    datas = img.getdata()

    newData = []
    for item in datas:
        # Check if the pixel is red-ish. 
        # Red crosshair is likely (255, 0, 0) or close.
        # We want to keep red pixels and make everything else transparent.
        
        r, g, b, a = item
        
        # Simple heuristic: If Red is dominant and high, keep it. 
        # Adjust thresholds as needed.
        if r > 150 and g < 100 and b < 100:
            newData.append(item) # Keep red
        else:
            newData.append((255, 255, 255, 0)) # Transparent

    img.putdata(newData)
    img.save(output_path, "PNG")
    print(f"Saved processed image to {output_path}")

if __name__ == "__main__":
    remove_background("src-tauri/icons/tray-icon.png", "src-tauri/icons/tray-icon-clean.png")
