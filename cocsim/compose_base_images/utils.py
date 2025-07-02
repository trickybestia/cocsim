from pathlib import Path

import PIL.Image


def load_base_images(
    path: Path,
) -> tuple[list[PIL.Image.Image], list[PIL.Image.Image]]:
    left = []
    right = []

    for file_path in path.iterdir():
        if file_path.name.startswith("l"):
            left.append((file_path.name, PIL.Image.open(file_path)))
        else:
            right.append((file_path.name, PIL.Image.open(file_path)))

    left.sort()
    right.sort()

    return [item[1] for item in left], [item[1] for item in right]
