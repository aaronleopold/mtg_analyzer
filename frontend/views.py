from django.shortcuts import render,redirect
from django.views.generic import TemplateView
from django.http import HttpResponseRedirect
# from django.db.models import Q
from mtgsdk import Card
# from scrython import *
import asyncio
import scrython
import timeit

def benchmark():
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    scrython_time = timeit.timeit(lambda: scrython.cards.Search(q="name:Slate Street Ruffian", order="spoiled"), number=1)
    loop.close()
    mtg_time = timeit.timeit(lambda: Card.where(name='Slate Street Ruffian').all(), number=1)
    print(scrython_time, mtg_time)

def search():
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    search = scrython.cards.Search(q="name:Slate Street Ruffian", order="spoiled")
    loop.close()

    return search.data()

def dev(request):
    # benchmark()

    cards = search()

    print(cards[0]['image_uris'])

    context = {
        'cards': cards
    }
    return render(request, 'cards.html', context)

