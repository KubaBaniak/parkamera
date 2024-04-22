import cv2
import numpy as np

FINAL_LINE_COLOR = (255, 255, 255)

class PolygonDrawer(object):
    def __init__(self, window_name, img):
        self.window_name = window_name
        self.img = img

        self.done = False
        self.current = (0, 0)
        self.polygons = []
        self.points = []

    def on_mouse(self, event, x, y, buttons, user_param):
        if self.done:
            return

        if event == cv2.EVENT_MOUSEMOVE:
            self.current = (x, y)
        elif event == cv2.EVENT_LBUTTONDOWN:
            print("Adding point #%d with position(%d,%d)" %
                  (len(self.points), x, y))
            self.points.append([x, y])
            if (len(self.points) == 4):
                self.polygons.append(self.points)
                self.points = []
        elif event == cv2.EVENT_RBUTTONDOWN:
            print("Completing polygon with %d polygons." % len(self.polygons))
            self.done = True

    def run(self):
        cv2.namedWindow(self.window_name)
        cv2.imshow(self.window_name, self.img)
        cv2.waitKey(1)
        cv2.setMouseCallback(self.window_name, self.on_mouse)

        while (not self.done):
            if (len(self.polygons) > 0):
                for polygon in self.polygons:
                    pts = np.array(polygon, np.int32).reshape((-1, 1, 2))
                    cv2.polylines(self.img, [pts], True, FINAL_LINE_COLOR, 1)

            cv2.imshow(self.window_name, self.img)
            if cv2.waitKey(50) == 27:
                self.done = True

        cv2.imshow(self.window_name, self.img)
        cv2.waitKey()

        cv2.destroyWindow(self.window_name)
        return self.polygons


def setup():
    img = cv2.imread("./images/image.jpg", cv2.IMREAD_COLOR)
    pd = PolygonDrawer("Polygon", img)

    coordinates = pd.run()
    with open('parking_spots_coordinates.txt', 'w') as file:
        for polygon in coordinates:
            for points in polygon:
                file.write(" ".join(str(point) for point in points))
                file.write(', ')
            file.write('\n')
        file.close()

    print("Polygon = %s" % pd.polygons)


if __name__ == "__main__":
    setup()
