from PIL import Image, ImageDraw, ImageTk
import tkinter as tk



def get_img(pictures, cv, w, h, my_tk):
    start_x, start_y = 200, 100

    img_canvas = Image.new('RGB', (w, h), 'white')
    ImageDraw.Draw(img_canvas)

    for (id, xo, yo) in pictures:
        x, y = start_x + xo, start_y + yo

        img = Image.open(id)
        img_canvas.paste(img, (x, y))

        cv.img = ImageTk.PhotoImage(img_canvas)
        cv.create_image(0, 0, anchor=tk.NW, image=cv.img)
        
        cv.update_idletasks()
        cv.after(20)
        my_tk.update_idletasks()  

def get_pictures():
    pictures = []
    with open("picture.txt", 'r') as f:
        ls = f.readlines()
        for l in ls:
            splitted = l.split(',')
            path, x, y = splitted[0], int(splitted[1]), int(splitted[2])  

            pictures.append((path, x, y))
    
    return pictures

if __name__ == "__main__":
    my_tk = tk.Tk()
    my_tk.title("Reconstructing image from pieces")
    pictures = get_pictures()
    w, h = 1000, 500

    cv = tk.Canvas(my_tk, width=w, height=h, bg="white")
    cv.pack()
    get_img(pictures, cv, w, h, my_tk)

    my_tk.mainloop()