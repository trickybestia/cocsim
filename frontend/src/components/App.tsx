import ComposeBaseImages from "./ComposeBaseImages";

function App() {
  return (
    <div className="p-4 w-[800px] h-[600px]">
      <ComposeBaseImages onComposed={(image) => console.log(image)} />
    </div>
  );
}

export default App;
