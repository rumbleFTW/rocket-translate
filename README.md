# **Rocket translate**

## Rocket-translate is a blazingly fast API for translation written in Rust rocket and rust-bert, It uses (T5)[https://huggingface.co/docs/transformers/model_doc/t5] checkpoints behind the stage, which can be replaced by any NLP pipeline supported by rust-bert.

## API Documentation:

1.  ### Home

    **_Endpoint:_**

    ```cURL
    GET http://127.0.0.1:8000/
    ```

    This endpoint provides general information about the API version and status.

2.  ### Languages

    **_Endpoint:_**

    ```cURL
    GET http://127.0.0.1:8000/languages
    ```

    This endpoint provides a list of supported languages.

    **_Response:_**

    - `languages`: An array of language codes.

3.  ### Translate

    **_Endpoint:_**

    ```cURL
    POST http://127.0.0.1:8000/translate
    ```

    **_Request body:_**
    Content-Type: raw (text)
    Example:

    ```json
    {
         "source_lang": "English",
         "target_lang": "French",
         "text": "This text will be translated to French"
    }
    ```

    **_Response:_**

    The response will include details about the translation:

    - `success`: A boolean indicating whether the translation was successful.
    - `source_language`: The detected or specified source language.
    - `target_language`: The specified target language.
    - `original_text`: The original text that was translated.
    - `translated_text`: The translated text.
