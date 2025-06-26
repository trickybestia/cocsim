import ComposeBaseImages from "../../components/ComposeBaseImages";

const Index: React.FC = () => {
  return <ComposeBaseImages className="p-4" onComposed={console.log} />;
};

export default Index;
