import { ChakraProvider } from "@chakra-ui/react";
import ShortenerForm from "./components/ShortenerForm";
import ResolverForm from "./components/ResolverForm";

function App() {
  return (
    <ChakraProvider>
      <div>
        <ShortenerForm />
        <ResolverForm />
      </div>
    </ChakraProvider>
  );
}

export default App;