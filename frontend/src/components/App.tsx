import ComposeBaseImages from "./ComposeBaseImages";

function App() {
  return (
    <ComposeBaseImages
      className="h-[600px] w-[800px] p-4"
      onComposed={console.log}
    />
  );
}

export default App;
