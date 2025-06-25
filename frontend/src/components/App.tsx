import ComposeBaseImages from "./ComposeBaseImages";

function App() {
  return (
    <div style={{ width: "1000px", height: "600px" }}>
      <ComposeBaseImages onComposed={(image) => console.log(image)} />
    </div>
  );
}

export default App;
