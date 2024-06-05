import streamlit as st
from qdrant_client import QdrantClient
from openai import OpenAI
from dotenv import load_dotenv
import pymongo
from bs4 import BeautifulSoup

load_dotenv()

qdrant = QdrantClient(host='localhost', port=6333)
client = OpenAI()

myclient = pymongo.MongoClient("mongodb://localhost:27017/")
mydb = myclient["ConfDatabase"]
mycol = mydb["ConfContent"]


def remove_empty_lines(text):
    lines = text.splitlines()
    non_empty_lines = [line for line in lines if line.strip()]
    cleaned_text = '\n'.join(non_empty_lines)
    return cleaned_text


def main():
    st.title("Chat With Your Confluence Space!")

    query = st.text_input("Enter your query:", "")

    if st.button("Search"):
        response = client.embeddings.create(
            input=query,
            model="text-embedding-3-small"
        )

        search_results = qdrant.search(collection_name="memory", query_vector=response.data[0].embedding, limit=1)
        results = ""

        if search_results:
            # st.write("Search Results:")
            # for result in search_results:
            #     st.write(result)
            for result in search_results:
                mydoc = mycol.find({"id": str(result.id)})
                for x in mydoc:
                    content = x['body']['storage']['value']
                    soup = BeautifulSoup(content, 'html.parser')
                    cleaned_text = remove_empty_lines(soup.get_text())
                    results = results + cleaned_text

            completion = client.chat.completions.create(
                model="gpt-4-turbo",
                messages=[
                    {"role": "system", "content": "You are an assistant who answers questions, say you do not know if the information is not in text provided here. Information:" + results},
                    {"role": "user", "content": query}
                ]
            )
            st.write(completion.choices[0].message.content)
        else:
            st.write("No results found.")


if __name__ == "__main__":
    main()
