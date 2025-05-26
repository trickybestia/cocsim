from pathlib import Path

import PIL.Image


def load_test_images(
    index: int,
) -> tuple[list[PIL.Image.Image], list[PIL.Image.Image]]:
    left = []
    right = []

    test_images = Path("test_images") / str(index)

    for file_path in test_images.iterdir():
        if file_path.name.startswith("l"):
            left.append((file_path.name, PIL.Image.open(file_path)))
        else:
            right.append((file_path.name, PIL.Image.open(file_path)))

    left.sort()
    right.sort()

    return [item[1] for item in left], [item[1] for item in right]
