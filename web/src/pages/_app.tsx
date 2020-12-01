import Head from "next/head";
import { AocProvider } from "../contexts/aoc";

const App = ({ Component, pageProps }: any) => (
  <>
    <Head>
      <title>Advent of Code 2020 - solutions</title>
      <meta name="Description" content="Solutions for Advent of Code 2020." />
    </Head>

    <AocProvider>
      <Component {...pageProps} />
    </AocProvider>
  </>
);

export default App;
