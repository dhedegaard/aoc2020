import {
  createContext,
  Reducer,
  useContext,
  useEffect,
  useReducer,
} from "react";

export type Aoc = typeof import("/home/dennis/src/aoc2020/aoc2020/pkg/aoc2020");

const AocContext = createContext<Aoc>(undefined as any);

export const AocProvider: React.FC = ({ children }) => {
  const [{ state, error, module }, dispatch] = useReducer<
    Reducer<
      | { state: "loading"; error?: undefined; module?: undefined }
      | { state: "done"; error?: undefined; module: Aoc }
      | { state: "error"; error: string; module?: undefined },
      { type: "SET_ERROR"; error: string } | { type: "SET_DONE"; module: Aoc }
    >
  >(
    (state, action) => {
      switch (action.type) {
        case "SET_ERROR":
          return {
            ...state,
            state: "error",
            module: undefined,
            error: action.error,
          };
        case "SET_DONE":
          return {
            ...state,
            state: "done",
            module: action.module,
            error: undefined,
          };
        default:
          return state;
      }
    },
    { state: "loading" }
  );

  useEffect(() => {
    import("../../../aoc2020/pkg/aoc2020")
      .then((module) => dispatch({ type: "SET_DONE", module }))
      .catch((error) => dispatch({ type: "SET_ERROR", error: error.message }));
  }, [dispatch]);

  return state === "loading" ? null : state === "error" ? (
    <div>Error: {error}</div>
  ) : (
    <AocContext.Provider value={module}>{children}</AocContext.Provider>
  );
};

export const useAoc = () => useContext(AocContext);
