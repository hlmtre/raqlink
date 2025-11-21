:: you can use this like so from cmd or pwsh:
:: request.bat <filename>
@echo off
curl --header 'Additional: Header' --form additional=image --form image=@%1 https://u.aql.ink/new_image