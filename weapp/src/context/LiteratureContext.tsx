// src/contexts/LiteratureContext.js
import { createContext, useState } from 'react';

export const LiteratureContext = createContext({
    selectedPdf: null,
    setSelectedPdf: (_: any) => { },
    // 可以在这里扩展，比如全局的Q&A列表，聊天记录等
});

export const LiteratureProvider = ({ children }) => {
    const [selectedPdf, setSelectedPdf] = useState(null);

    return (
        <LiteratureContext.Provider value={{ selectedPdf, setSelectedPdf }}>
            {children}
        </LiteratureContext.Provider>
    );
};