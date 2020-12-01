import { NextPage } from "next";
import { useCallback, useState } from "react";
import { useAoc } from "../contexts/aoc";
import styled from "styled-components";

const Container = styled.div`
  max-width: 800px;
  margin: 8px auto;
`;

const Row = styled.div`
  display: flex;
  justify-content: space-between;
`;

const Column = styled.div`
  display: flex;
  flex-direction: column;
  width: 40%;
`;

const Index: NextPage = () => {
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");

  const onChangeInput = useCallback(
    (event: React.ChangeEvent<HTMLTextAreaElement>) => {
      setInput(event.target.value);
    },
    [setInput]
  );

  const aoc = useAoc();
  return (
    <Container>
      <Row>
        <Column>
          <label htmlFor="id_input">Input:</label>
          <textarea
            id="id_input"
            value={input}
            rows={10}
            onChange={onChangeInput}
          />
        </Column>
        <Column>
          <label htmlFor="id_output">Output:</label>
          <textarea id="id_output" value={output} rows={10} readOnly />
        </Column>
      </Row>
      <hr />
      <Row>
        <table>
          <thead>
            <tr>
              <th>Day</th>
              <th>Input</th>
              <th>Part 1</th>
              <th>Part 2</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>1</td>
              <td>
                <button onClick={() => setInput(aoc.day01_input())}>
                  Load
                </button>
              </td>
              <td>
                <button onClick={() => setOutput(aoc.day01_part1(input))}>
                  Solve
                </button>
              </td>
              <td>
                <button onClick={() => setOutput(aoc.day01_part2(input))}>
                  Solve
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </Row>
    </Container>
  );
};

export default Index;
