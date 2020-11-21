import { NextPage } from "next";
import { useAoc } from "../contexts/aoc";

const Index: NextPage = () => {
  const aoc = useAoc();
  return (
    <div>
      <button onClick={() => aoc.greet()}>HEJ</button>
      TODO:
    </div>
  );
};

export default Index;
