import streamlit as st
from qdrant_client import QdrantClient

qdrant = QdrantClient(host='localhost', port=6333)

def main():
    st.title("RAG Chatbot with Qdrant")

    query = st.text_input("Enter your query:", "")

    if st.button("Search"):
        search_results = qdrant.search(query)

        if search_results:
            st.write("Search Results:")
            for result in search_results:
                st.write(result)
        else:
            st.write("No results found.")

if __name__ == "__main__":
    main()
