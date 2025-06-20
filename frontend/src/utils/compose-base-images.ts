const composeBaseImages = (left: Blob[], right: Blob[]): Blob => {
  const image = left[0];

  console.log(image);

  console.log(nj.images.read(image));
};

export default composeBaseImages;
