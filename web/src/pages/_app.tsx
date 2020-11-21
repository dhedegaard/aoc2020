import { AocProvider } from "../contexts/aoc";

const App = ({ Component, pageProps }: any) => (
  <AocProvider>
    <Component {...pageProps} />
  </AocProvider>
);

export default App;
