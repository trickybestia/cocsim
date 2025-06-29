const readFiles = (
  callback: (files: Blob[]) => void,
  accept: string,
  multiple: boolean = true
) => {
  const fileInput = document.createElement("input");

  fileInput.setAttribute("type", "file");
  fileInput.setAttribute("accept", accept);

  if (multiple) {
    fileInput.setAttribute("multiple", "");
  }

  fileInput.onchange = () => {
    if (fileInput.files === null || fileInput.files.length === 0) return;

    const files = [];

    for (let i = 0; i != fileInput.files.length; i++) {
      files.push(fileInput.files.item(i)!);
    }

    callback(files);
  };

  fileInput.click();
};

export default readFiles;
